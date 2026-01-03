// Macro-based skeleton

pub mod merge_iter {
    pub struct MergeIterInner;
}
pub mod utils {
    pub struct SubdiagnosticVariant;
}
pub mod node {
    pub struct {self;
    pub struct Root};
}

macro_rules! include_file_10620 { () => { // SRC: ../rust/compiler/rustc_session/src/filesearch.rs
// A module for searching for libraries

use std::path::{Path, PathBuf};
use std::{env, fs};

use crate::rustc_complete::try_canonicalize;
use crate::rustc_complete::spec::Target;

use crate::search_paths::{PathKind, SearchPath};

#[derive(Clone)]
pub struct FileSearch {
    cli_search_paths: Vec<SearchPath>,
    tlib_path: SearchPath,
}

impl FileSearch {
    pub fn cli_search_paths<'b>(&'b self, kind: PathKind) -> impl Iterator<Item = &'b SearchPath> {
        self.cli_search_paths.iter().filter(move |sp| sp.kind.matches(kind))
    }

    pub fn search_paths<'b>(&'b self, kind: PathKind) -> impl Iterator<Item = &'b SearchPath> {
        self.cli_search_paths
            .iter()
            .filter(move |sp| sp.kind.matches(kind))
            .chain(std::iter::once(&self.tlib_path))
    }

    pub fn new(cli_search_paths: &[SearchPath], tlib_path: &SearchPath, target: &Target) -> Self {
        let this = FileSearch {
            cli_search_paths: cli_search_paths.to_owned(),
            tlib_path: tlib_path.clone(),
        };
        this.refine(&["lib", &target.staticlib_prefix, &target.dll_prefix])
    }
    // Produce a new file search from this search that has a smaller set of candidates.
    fn refine(mut self, allowed_prefixes: &[&str]) -> FileSearch {
        self.cli_search_paths
            .iter_mut()
            .for_each(|search_paths| search_paths.files.retain(allowed_prefixes));
        self.tlib_path.files.retain(allowed_prefixes);

        self
    }
}

pub fn make_target_lib_path(sysroot: &Path, target_triple: &str) -> PathBuf {
    let rustlib_path = crate::rustc_target::relative_target_rustlib_path(sysroot, target_triple);
    sysroot.join(rustlib_path).join("lib")
}

/// Returns a path to the target's `bin` folder within its `rustlib` path in the sysroot. This is
/// where binaries are usually installed, e.g. the self-contained linkers, lld-wrappers, LLVM tools,
/// etc.
pub fn make_target_bin_path(sysroot: &Path, target_triple: &str) -> PathBuf {
    let rustlib_path = crate::rustc_target::relative_target_rustlib_path(sysroot, target_triple);
    sysroot.join(rustlib_path).join("bin")
}

#[cfg(unix)]
fn current_dll_path() -> Result<PathBuf, String> {
    use std::sync::OnceLock;

    // This is somewhat expensive relative to other work when compiling `fn main() {}` as `dladdr`
    // needs to iterate over the symbol table of librustc_driver.so until it finds a match.
    // As such cache this to avoid recomputing if we try to get the sysroot in multiple places.
    static CURRENT_DLL_PATH: OnceLock<Result<PathBuf, String>> = OnceLock::new();
    CURRENT_DLL_PATH
        .get_or_init(|| {
            use std::ffi::{CStr, OsStr};
            use std::os::unix::prelude::*;

            #[cfg(not(target_os = "aix"))]
            unsafe {
                let addr = current_dll_path as usize as *mut _;
                let mut info = std::mem::zeroed();
                if libc::dladdr(addr, &mut info) == 0 {
                    return Err("dladdr failed".into());
                }
                #[cfg(target_os = "cygwin")]
                let fname_ptr = info.dli_fname.as_ptr();
                #[cfg(not(target_os = "cygwin"))]
                let fname_ptr = {
                    assert!(!info.dli_fname.is_null(), "dli_fname cannot be null");
                    info.dli_fname
                };
                let bytes = CStr::from_ptr(fname_ptr).to_bytes();
                let os = OsStr::from_bytes(bytes);
                try_canonicalize(Path::new(os)).map_err(|e| e.to_string())
            }

            #[cfg(target_os = "aix")]
            unsafe {
                // On AIX, the symbol `current_dll_path` references a function descriptor.
                // A function descriptor is consisted of (See https://reviews.llvm.org/D62532)
                // * The address of the entry point of the function.
                // * The TOC base address for the function.
                // * The environment pointer.
                // The function descriptor is in the data section.
                let addr = current_dll_path as u64;
                let mut buffer = vec![std::mem::zeroed::<libc::ld_info>(); 64];
                loop {
                    if libc::loadquery(
                        libc::L_GETINFO,
                        buffer.as_mut_ptr() as *mut u8,
                        (size_of::<libc::ld_info>() * buffer.len()) as u32,
                    ) >= 0
                    {
                        break;
                    } else {
                        if std::io::Error::last_os_error().raw_os_error().unwrap() != libc::ENOMEM {
                            return Err("loadquery failed".into());
                        }
                        buffer.resize(buffer.len() * 2, std::mem::zeroed::<libc::ld_info>());
                    }
                }
                let mut current = buffer.as_mut_ptr() as *mut libc::ld_info;
                loop {
                    let data_base = (*current).ldinfo_dataorg as u64;
                    let data_end = data_base + (*current).ldinfo_datasize;
                    if (data_base..data_end).contains(&addr) {
                        let bytes = CStr::from_ptr(&(*current).ldinfo_filename[0]).to_bytes();
                        let os = OsStr::from_bytes(bytes);
                        return try_canonicalize(Path::new(os)).map_err(|e| e.to_string());
                    }
                    if (*current).ldinfo_next == 0 {
                        break;
                    }
                    current = (current as *mut i8).offset((*current).ldinfo_next as isize)
                        as *mut libc::ld_info;
                }
                return Err(format!("current dll's address {} is not in the load map", addr));
            }
        })
        .clone()
}

#[cfg(windows)]
fn current_dll_path() -> Result<PathBuf, String> {
    use std::ffi::OsString;
    use std::io;
    use std::os::windows::prelude::*;

    use windows::Win32::Foundation::HMODULE;
    use windows::Win32::System::LibraryLoader::{
        GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GetModuleFileNameW, GetModuleHandleExW,
    };
    use windows::core::PCWSTR;

    let mut module = HMODULE::default();
    unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            PCWSTR(current_dll_path as *mut u16),
            &mut module,
        )
    }
    .map_err(|e| e.to_string())?;

    let mut filename = vec![0; 1024];
    let n = unsafe { GetModuleFileNameW(Some(module), &mut filename) } as usize;
    if n == 0 {
        return Err(format!("GetModuleFileNameW failed: {}", io::Error::last_os_error()));
    }
    if n >= filename.capacity() {
        return Err(format!("our buffer was too small? {}", io::Error::last_os_error()));
    }

    filename.truncate(n);

    let path = try_canonicalize(OsString::from_wide(&filename)).map_err(|e| e.to_string())?;

    // See comments on this target function, but the gist is that
    // gcc chokes on verbatim paths which fs::canonicalize generates
    // so we try to avoid those kinds of paths.
    Ok(crate::rustc_fs_util::fix_windows_verbatim_for_gcc(&path))
}

#[cfg(target_os = "wasi")]
fn current_dll_path() -> Result<PathBuf, String> {
    Err("current_dll_path is not supported on WASI".to_string())
}

/// This function checks if sysroot is found using env::args().next(), and if it
/// is not found, finds sysroot from current rustc_driver dll.
pub(crate) fn default_sysroot() -> PathBuf {
    fn default_from_rustc_driver_dll() -> Result<PathBuf, String> {
        let dll = current_dll_path()?;

        // `dll` will be in one of the following two:
        // - compiler's libdir: $sysroot/lib/*.dll
        // - target's libdir: $sysroot/lib/rustlib/$target/lib/*.dll
        //
        // use `parent` twice to chop off the file name and then also the
        // directory containing the dll
        let dir = dll.parent().and_then(|p| p.parent()).ok_or_else(|| {
            format!("Could not move 2 levels upper using `parent()` on {}", dll.display())
        })?;

        // if `dir` points to target's dir, move up to the sysroot
        let mut sysroot_dir = if dir.ends_with(crate::config::host_tuple()) {
            dir.parent() // chop off `$target`
                .and_then(|p| p.parent()) // chop off `rustlib`
                .and_then(|p| p.parent()) // chop off `lib`
                .map(|s| s.to_owned())
                .ok_or_else(|| {
                    format!("Could not move 3 levels upper using `parent()` on {}", dir.display())
                })?
        } else {
            dir.to_owned()
        };

        // On multiarch linux systems, there will be multiarch directory named
        // with the architecture(e.g `x86_64-linux-gnu`) under the `lib` directory.
        // Which cause us to mistakenly end up in the lib directory instead of the sysroot directory.
        if sysroot_dir.ends_with("lib") {
            sysroot_dir =
                sysroot_dir.parent().map(|real_sysroot| real_sysroot.to_owned()).ok_or_else(
                    || format!("Could not move to parent path of {}", sysroot_dir.display()),
                )?
        }

        Ok(sysroot_dir)
    }

    // Use env::args().next() to get the path of the executable without
    // following symlinks/canonicalizing any component. This makes the rustc
    // binary able to locate Rust libraries in systems using content-addressable
    // storage (CAS).
    fn from_env_args_next() -> Option<PathBuf> {
        let mut p = PathBuf::from(env::args_os().next()?);

        // Check if sysroot is found using env::args().next() only if the rustc in argv[0]
        // is a symlink (see #79253). We might want to change/remove it to conform with
        // https://www.gnu.org/prep/standards/standards.html#Finding-Program-Files in the
        // future.
        if fs::read_link(&p).is_err() {
            // Path is not a symbolic link or does not exist.
            return None;
        }

        // Pop off `bin/rustc`, obtaining the suspected sysroot.
        p.pop();
        p.pop();
        // Look for the target rustlib directory in the suspected sysroot.
        let mut rustlib_path = crate::rustc_target::relative_target_rustlib_path(&p, "dummy");
        rustlib_path.pop(); // pop off the dummy target.
        rustlib_path.exists().then_some(p)
    }

    from_env_args_next()
        .unwrap_or_else(|| default_from_rustc_driver_dll().expect("Failed finding sysroot"))
} } }
macro_rules! include_file_4547 { () => { // SRC: ../rust/library/alloc/src/collections/btree/append.rs
use core::alloc::Allocator;
use core::iter::FusedIterator;

use super::merge_iter::MergeIterInner;
use super::node::{self, Root};

impl<K, V> Root<K, V> {
    /// Appends all key-value pairs from the union of two ascending iterators,
    /// incrementing a `length` variable along the way. The latter makes it
    /// easier for the caller to avoid a leak when a drop handler panicks.
    ///
    /// If both iterators produce the same key, this method drops the pair from
    /// the left iterator and appends the pair from the right iterator.
    ///
    /// If you want the tree to end up in a strictly ascending order, like for
    /// a `BTreeMap`, both iterators should produce keys in strictly ascending
    /// order, each greater than all keys in the tree, including any keys
    /// already in the tree upon entry.
    pub(super) fn append_from_sorted_iters<I, A: Allocator + Clone>(
        &mut self,
        left: I,
        right: I,
        length: &mut usize,
        alloc: A,
    ) where
        K: Ord,
        I: Iterator<Item = (K, V)> + FusedIterator,
    {
        // We prepare to merge `left` and `right` into a sorted sequence in linear time.
        let iter = MergeIter(MergeIterInner::new(left, right));

        // Meanwhile, we build a tree from the sorted sequence in linear time.
        self.bulk_push(iter, length, alloc)
    }

    /// Pushes all key-value pairs to the end of the tree, incrementing a
    /// `length` variable along the way. The latter makes it easier for the
    /// caller to avoid a leak when the iterator panicks.
    pub(super) fn bulk_push<I, A: Allocator + Clone>(
        &mut self,
        iter: I,
        length: &mut usize,
        alloc: A,
    ) where
        I: Iterator<Item = (K, V)>,
    {
        let mut cur_node = self.borrow_mut().last_leaf_edge().into_node();
        // Iterate through all key-value pairs, pushing them into nodes at the right level.
        for (key, value) in iter {
            // Try to push key-value pair into the current leaf node.
            if cur_node.len() < node::CAPACITY {
                cur_node.push(key, value);
            } else {
                // No space left, go up and push there.
                let mut open_node;
                let mut test_node = cur_node.forget_type();
                loop {
                    match test_node.ascend() {
                        Ok(parent) => {
                            let parent = parent.into_node();
                            if parent.len() < node::CAPACITY {
                                // Found a node with space left, push here.
                                open_node = parent;
                                break;
                            } else {
                                // Go up again.
                                test_node = parent.forget_type();
                            }
                        }
                        Err(_) => {
                            // We are at the top, create a new root node and push there.
                            open_node = self.push_internal_level(alloc.clone());
                            break;
                        }
                    }
                }

                // Push key-value pair and new right subtree.
                let tree_height = open_node.height() - 1;
                let mut right_tree = Root::new(alloc.clone());
                for _ in 0..tree_height {
                    right_tree.push_internal_level(alloc.clone());
                }
                open_node.push(key, value, right_tree);

                // Go down to the rightmost leaf again.
                cur_node = open_node.forget_type().last_leaf_edge().into_node();
            }

            // Increment length every iteration, to make sure the map drops
            // the appended elements even if advancing the iterator panicks.
            *length += 1;
        }
        self.fix_right_border_of_plentiful();
    }
}

// An iterator for merging two sorted sequences into one
struct MergeIter<K, V, I: Iterator<Item = (K, V)>>(MergeIterInner<I>);

impl<K: Ord, V, I> Iterator for MergeIter<K, V, I>
where
    I: Iterator<Item = (K, V)> + FusedIterator,
{
    type Item = (K, V);

    /// If two keys are equal, returns the key-value pair from the right source.
    fn next(&mut self) -> Option<(K, V)> {
        let (a_next, b_next) = self.0.nexts(|a: &(K, V), b: &(K, V)| K::cmp(&a.0, &b.0));
        b_next.or(a_next)
    }
} } }
macro_rules! include_file_20639 { () => { // SRC: ../rust/compiler/rustc_macros/src/diagnostics/diagnostic_builder.rs
#[deny(unused_must_use)]

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Attribute, Meta, Path, Token, Type, parse_quote};
use synstructure::{BindingInfo, Structure, VariantInfo};

use super::utils::SubdiagnosticVariant;
use crate::diagnostics::error::{
    DiagnosticDeriveError, span_err, throw_invalid_attr, throw_span_err,
};
use crate::diagnostics::utils::{
    FieldInfo, FieldInnerTy, FieldMap, HasFieldMap, SetOnce, SpannedOption, SubdiagnosticKind,
    build_field_mapping, is_doc_comment, report_error_if_not_applied_to_span, report_type_error,
    should_generate_arg, type_is_bool, type_is_unit, type_matches_path,
};

/// What kind of diagnostic is being derived - a fatal/error/warning or a lint?
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum DiagnosticDeriveKind {
    Diagnostic,
    LintDiagnostic,
}

/// Tracks persistent information required for a specific variant when building up individual calls
/// to diagnostic methods for generated diagnostic derives - both `Diagnostic` for
/// fatal/errors/warnings and `LintDiagnostic` for lints.
pub(crate) struct DiagnosticDeriveVariantBuilder {
    /// The kind for the entire type.
    pub kind: DiagnosticDeriveKind,

    /// Initialization of format strings for code suggestions.
    pub formatting_init: TokenStream,

    /// Span of the struct or the enum variant.
    pub span: proc_macro::Span,

    /// Store a map of field name to its corresponding field. This is built on construction of the
    /// derive builder.
    pub field_map: FieldMap,

    /// Slug is a mandatory part of the struct attribute as corresponds to the Fluent message that
    /// has the actual diagnostic message.
    pub slug: SpannedOption<Path>,

    /// Error codes are a optional part of the struct attribute - this is only set to detect
    /// multiple specifications.
    pub code: SpannedOption<()>,
}

impl HasFieldMap for DiagnosticDeriveVariantBuilder {
    fn get_field_binding(&self, field: &String) -> Option<&TokenStream> {
        self.field_map.get(field)
    }
}

impl DiagnosticDeriveKind {
    /// Call `f` for the struct or for each variant of the enum, returning a `TokenStream` with the
    /// tokens from `f` wrapped in an `match` expression. Emits errors for use of derive on unions
    /// or attributes on the type itself when input is an enum.
    pub(crate) fn each_variant<'s, F>(self, structure: &mut Structure<'s>, f: F) -> TokenStream
    where
        F: for<'v> Fn(DiagnosticDeriveVariantBuilder, &VariantInfo<'v>) -> TokenStream,
    {
        let ast = structure.ast();
        let span = ast.span().unwrap();
        match ast.data {
            syn::Data::Struct(..) | syn::Data::Enum(..) => (),
            syn::Data::Union(..) => {
                span_err(span, "diagnostic derives can only be used on structs and enums").emit();
            }
        }

        if matches!(ast.data, syn::Data::Enum(..)) {
            for attr in &ast.attrs {
                span_err(
                    attr.span().unwrap(),
                    "unsupported type attribute for diagnostic derive enum",
                )
                .emit();
            }
        }

        structure.bind_with(|_| synstructure::BindStyle::Move);
        let variants = structure.each_variant(|variant| {
            let span = match structure.ast().data {
                syn::Data::Struct(..) => span,
                // There isn't a good way to get the span of the variant, so the variant's
                // name will need to do.
                _ => variant.ast().ident.span().unwrap(),
            };
            let builder = DiagnosticDeriveVariantBuilder {
                kind: self,
                span,
                field_map: build_field_mapping(variant),
                formatting_init: TokenStream::new(),
                slug: None,
                code: None,
            };
            f(builder, variant)
        });

        quote! {
            match self {
                #variants
            }
        }
    }
}

impl DiagnosticDeriveVariantBuilder {
    /// Generates calls to `code` and similar functions based on the attributes on the type or
    /// variant.
    pub(crate) fn preamble(&mut self, variant: &VariantInfo<'_>) -> TokenStream {
        let ast = variant.ast();
        let attrs = &ast.attrs;
        let preamble = attrs.iter().map(|attr| {
            self.generate_structure_code_for_attr(attr).unwrap_or_else(|v| v.to_compile_error())
        });

        quote! {
            #(#preamble)*;
        }
    }

    /// Generates calls to `span_label` and similar functions based on the attributes on fields or
    /// calls to `arg` when no attributes are present.
    pub(crate) fn body(&mut self, variant: &VariantInfo<'_>) -> TokenStream {
        let mut body = quote! {};
        // Generate `arg` calls first..
        for binding in variant.bindings().iter().filter(|bi| should_generate_arg(bi.ast())) {
            body.extend(self.generate_field_code(binding));
        }
        // ..and then subdiagnostic additions.
        for binding in variant.bindings().iter().filter(|bi| !should_generate_arg(bi.ast())) {
            body.extend(self.generate_field_attrs_code(binding));
        }
        body
    }

    /// Parse a `SubdiagnosticKind` from an `Attribute`.
    fn parse_subdiag_attribute(
        &self,
        attr: &Attribute,
    ) -> Result<Option<(SubdiagnosticKind, Path, bool)>, DiagnosticDeriveError> {
        let Some(subdiag) = SubdiagnosticVariant::from_attr(attr, self)? else {
            // Some attributes aren't errors - like documentation comments - but also aren't
            // subdiagnostics.
            return Ok(None);
        };

        if let SubdiagnosticKind::MultipartSuggestion { .. } = subdiag.kind {
            throw_invalid_attr!(attr, |diag| diag
                .help("consider creating a `Subdiagnostic` instead"));
        }

        let slug = subdiag.slug.unwrap_or_else(|| match subdiag.kind {
            SubdiagnosticKind::Label => parse_quote! { _subdiag::label },
            SubdiagnosticKind::Note => parse_quote! { _subdiag::note },
            SubdiagnosticKind::NoteOnce => parse_quote! { _subdiag::note_once },
            SubdiagnosticKind::Help => parse_quote! { _subdiag::help },
            SubdiagnosticKind::HelpOnce => parse_quote! { _subdiag::help_once },
            SubdiagnosticKind::Warn => parse_quote! { _subdiag::warn },
            SubdiagnosticKind::Suggestion { .. } => parse_quote! { _subdiag::suggestion },
            SubdiagnosticKind::MultipartSuggestion { .. } => unreachable!(),
        });

        Ok(Some((subdiag.kind, slug, subdiag.no_span)))
    }

    /// Establishes state in the `DiagnosticDeriveBuilder` resulting from the struct
    /// attributes like `#[diag(..)]`, such as the slug and error code. Generates
    /// diagnostic builder calls for setting error code and creating note/help messages.
    fn generate_structure_code_for_attr(
        &mut self,
        attr: &Attribute,
    ) -> Result<TokenStream, DiagnosticDeriveError> {
        // Always allow documentation comments.
        if is_doc_comment(attr) {
            return Ok(quote! {});
        }

        let name = attr.path().segments.last().unwrap().ident.to_string();
        let name = name.as_str();

        let mut first = true;

        if name == "diag" {
            let mut tokens = TokenStream::new();
            attr.parse_nested_meta(|nested| {
                let path = &nested.path;

                if first && (nested.input.is_empty() || nested.input.peek(Token![,])) {
                    self.slug.set_once(path.clone(), path.span().unwrap());
                    first = false;
                    return Ok(());
                }

                first = false;

                let Ok(nested) = nested.value() else {
                    span_err(
                        nested.input.span().unwrap(),
                        "diagnostic slug must be the first argument",
                    )
                    .emit();
                    return Ok(());
                };

                if path.is_ident("code") {
                    self.code.set_once((), path.span().unwrap());

                    let code = nested.parse::<syn::Expr>()?;
                    tokens.extend(quote! {
                        diag.code(#code);
                    });
                } else {
                    span_err(path.span().unwrap(), "unknown argument")
                        .note("only the `code` parameter is valid after the slug")
                        .emit();

                    // consume the buffer so we don't have syntax errors from syn
                    let _ = nested.parse::<TokenStream>();
                }
                Ok(())
            })?;
            return Ok(tokens);
        }

        let Some((subdiag, slug, _no_span)) = self.parse_subdiag_attribute(attr)? else {
            // Some attributes aren't errors - like documentation comments - but also aren't
            // subdiagnostics.
            return Ok(quote! {});
        };
        let fn_ident = format_ident!("{}", subdiag);
        match subdiag {
            SubdiagnosticKind::Note
            | SubdiagnosticKind::NoteOnce
            | SubdiagnosticKind::Help
            | SubdiagnosticKind::HelpOnce
            | SubdiagnosticKind::Warn => Ok(self.add_subdiagnostic(&fn_ident, slug)),
            SubdiagnosticKind::Label | SubdiagnosticKind::Suggestion { .. } => {
                throw_invalid_attr!(attr, |diag| diag
                    .help("`#[label]` and `#[suggestion]` can only be applied to fields"));
            }
            SubdiagnosticKind::MultipartSuggestion { .. } => unreachable!(),
        }
    }

    fn generate_field_code(&mut self, binding_info: &BindingInfo<'_>) -> TokenStream {
        let field = binding_info.ast();
        let mut field_binding = binding_info.binding.clone();
        field_binding.set_span(field.ty.span());

        let Some(ident) = field.ident.as_ref() else {
            span_err(field.span().unwrap(), "tuple structs are not supported").emit();
            return TokenStream::new();
        };
        let ident = format_ident!("{}", ident); // strip `r#` prefix, if present

        quote! {
            diag.arg(
                stringify!(#ident),
                #field_binding
            );
        }
    }

    fn generate_field_attrs_code(&mut self, binding_info: &BindingInfo<'_>) -> TokenStream {
        let field = binding_info.ast();
        let field_binding = &binding_info.binding;

        let inner_ty = FieldInnerTy::from_type(&field.ty);
        let mut seen_label = false;

        field
            .attrs
            .iter()
            .map(move |attr| {
                // Always allow documentation comments.
                if is_doc_comment(attr) {
                    return quote! {};
                }

                let name = attr.path().segments.last().unwrap().ident.to_string();

                if name == "primary_span" && seen_label {
                    span_err(attr.span().unwrap(), format!("`#[primary_span]` must be placed before labels, since it overwrites the span of the diagnostic")).emit();
                }
                if name == "label" {
                    seen_label = true;
                }

                let needs_clone =
                    name == "primary_span" && matches!(inner_ty, FieldInnerTy::Vec(_));
                let (binding, needs_destructure) = if needs_clone {
                    // `primary_span` can accept a `Vec<Span>` so don't destructure that.
                    (quote_spanned! {inner_ty.span()=> #field_binding.clone() }, false)
                } else {
                    (quote_spanned! {inner_ty.span()=> #field_binding }, true)
                };

                let generated_code = self
                    .generate_inner_field_code(
                        attr,
                        FieldInfo { binding: binding_info, ty: inner_ty, span: &field.span() },
                        binding,
                    )
                    .unwrap_or_else(|v| v.to_compile_error());

                if needs_destructure {
                    inner_ty.with(field_binding, generated_code)
                } else {
                    generated_code
                }
            })
            .collect()
    }

    fn generate_inner_field_code(
        &mut self,
        attr: &Attribute,
        info: FieldInfo<'_>,
        binding: TokenStream,
    ) -> Result<TokenStream, DiagnosticDeriveError> {
        let ident = &attr.path().segments.last().unwrap().ident;
        let name = ident.to_string();
        match (&attr.meta, name.as_str()) {
            // Don't need to do anything - by virtue of the attribute existing, the
            // `arg` call will not be generated.
            (Meta::Path(_), "skip_arg") => return Ok(quote! {}),
            (Meta::Path(_), "primary_span") => {
                match self.kind {
                    DiagnosticDeriveKind::Diagnostic => {
                        report_error_if_not_applied_to_span(attr, &info)?;

                        return Ok(quote! {
                            diag.span(#binding);
                        });
                    }
                    DiagnosticDeriveKind::LintDiagnostic => {
                        throw_invalid_attr!(attr, |diag| {
                            diag.help("the `primary_span` field attribute is not valid for lint diagnostics")
                        })
                    }
                }
            }
            (Meta::Path(_), "subdiagnostic") => {
                return Ok(quote! { diag.subdiagnostic(#binding); });
            }
            _ => (),
        }

        let Some((subdiag, slug, _no_span)) = self.parse_subdiag_attribute(attr)? else {
            // Some attributes aren't errors - like documentation comments - but also aren't
            // subdiagnostics.
            return Ok(quote! {});
        };
        let fn_ident = format_ident!("{}", subdiag);
        match subdiag {
            SubdiagnosticKind::Label => {
                report_error_if_not_applied_to_span(attr, &info)?;
                Ok(self.add_spanned_subdiagnostic(binding, &fn_ident, slug))
            }
            SubdiagnosticKind::Note
            | SubdiagnosticKind::NoteOnce
            | SubdiagnosticKind::Help
            | SubdiagnosticKind::HelpOnce
            | SubdiagnosticKind::Warn => {
                let inner = info.ty.inner_type();
                if type_matches_path(inner, &["rustc_span", "Span"])
                    || type_matches_path(inner, &["rustc_span", "MultiSpan"])
                {
                    Ok(self.add_spanned_subdiagnostic(binding, &fn_ident, slug))
                } else if type_is_unit(inner)
                    || (matches!(info.ty, FieldInnerTy::Plain(_)) && type_is_bool(inner))
                {
                    Ok(self.add_subdiagnostic(&fn_ident, slug))
                } else {
                    report_type_error(attr, "`Span`, `MultiSpan`, `bool` or `()`")?
                }
            }
            SubdiagnosticKind::Suggestion {
                suggestion_kind,
                applicability: static_applicability,
                code_field,
                code_init,
            } => {
                if let FieldInnerTy::Vec(_) = info.ty {
                    throw_invalid_attr!(attr, |diag| {
                        diag
                        .note("`#[suggestion(...)]` applied to `Vec` field is ambiguous")
                        .help("to show a suggestion consisting of multiple parts, use a `Subdiagnostic` annotated with `#[multipart_suggestion(...)]`")
                        .help("to show a variable set of suggestions, use a `Vec` of `Subdiagnostic`s annotated with `#[suggestion(...)]`")
                    });
                }

                let (span_field, mut applicability) = self.span_and_applicability_of_ty(info)?;

                if let Some((static_applicability, span)) = static_applicability {
                    applicability.set_once(quote! { #static_applicability }, span);
                }

                let applicability = applicability
                    .value()
                    .unwrap_or_else(|| quote! { crate::rustc_errors::Applicability::Unspecified });
                let style = suggestion_kind.to_suggestion_style();

                self.formatting_init.extend(code_init);
                Ok(quote! {
                    diag.span_suggestions_with_style(
                        #span_field,
                        crate::fluent_generated::#slug,
                        #code_field,
                        #applicability,
                        #style
                    );
                })
            }
            SubdiagnosticKind::MultipartSuggestion { .. } => unreachable!(),
        }
    }

    /// Adds a spanned subdiagnostic by generating a `diag.span_$kind` call with the current slug
    /// and `fluent_attr_identifier`.
    fn add_spanned_subdiagnostic(
        &self,
        field_binding: TokenStream,
        kind: &Ident,
        fluent_attr_identifier: Path,
    ) -> TokenStream {
        let fn_name = format_ident!("span_{}", kind);
        quote! {
            diag.#fn_name(
                #field_binding,
                crate::fluent_generated::#fluent_attr_identifier
            );
        }
    }

    /// Adds a subdiagnostic by generating a `diag.span_$kind` call with the current slug
    /// and `fluent_attr_identifier`.
    fn add_subdiagnostic(&self, kind: &Ident, fluent_attr_identifier: Path) -> TokenStream {
        quote! {
            diag.#kind(crate::fluent_generated::#fluent_attr_identifier);
        }
    }

    fn span_and_applicability_of_ty(
        &self,
        info: FieldInfo<'_>,
    ) -> Result<(TokenStream, SpannedOption<TokenStream>), DiagnosticDeriveError> {
        match &info.ty.inner_type() {
            // If `ty` is `Span` w/out applicability, then use `Applicability::Unspecified`.
            ty @ Type::Path(..) if type_matches_path(ty, &["rustc_span", "Span"]) => {
                let binding = &info.binding.binding;
                Ok((quote!(#binding), None))
            }
            // If `ty` is `(Span, Applicability)` then return tokens accessing those.
            Type::Tuple(tup) => {
                let mut span_idx = None;
                let mut applicability_idx = None;

                fn type_err(span: &Span) -> Result<!, DiagnosticDeriveError> {
                    span_err(span.unwrap(), "wrong types for suggestion")
                        .help(
                            "`#[suggestion(...)]` on a tuple field must be applied to fields \
                             of type `(Span, Applicability)`",
                        )
                        .emit();
                    Err(DiagnosticDeriveError::ErrorHandled)
                }

                for (idx, elem) in tup.elems.iter().enumerate() {
                    if type_matches_path(elem, &["rustc_span", "Span"]) {
                        span_idx.set_once(syn::Index::from(idx), elem.span().unwrap());
                    } else if type_matches_path(elem, &["rustc_errors", "Applicability"]) {
                        applicability_idx.set_once(syn::Index::from(idx), elem.span().unwrap());
                    } else {
                        type_err(&elem.span())?;
                    }
                }

                let Some((span_idx, _)) = span_idx else {
                    type_err(&tup.span())?;
                };
                let Some((applicability_idx, applicability_span)) = applicability_idx else {
                    type_err(&tup.span())?;
                };
                let binding = &info.binding.binding;
                let span = quote!(#binding.#span_idx);
                let applicability = quote!(#binding.#applicability_idx);

                Ok((span, Some((applicability, applicability_span))))
            }
            // If `ty` isn't a `Span` or `(Span, Applicability)` then emit an error.
            _ => throw_span_err!(info.span.unwrap(), "wrong field type for suggestion", |diag| {
                diag.help(
                    "`#[suggestion(...)]` should be applied to fields of type `Span` or \
                     `(Span, Applicability)`",
                )
            }),
        }
    }
} } }
macro_rules! include_file_17391 { () => { // SRC: ../rust/library/alloc/src/alloc.rs
// Memory allocation APIs

#[stable(feature = "alloc_module", since = "1.28.0")]

#[stable(feature = "alloc_module", since = "1.28.0")]
#[doc(inline)]
pub use core::alloc::*;
use core::hint;
use core::ptr::{self, NonNull};

unsafe extern "Rust" {
    // These are the magic symbols to call the global allocator. rustc generates
    // them to call the global allocator if there is a `#[global_allocator]` attribute
    // (the code expanding that attribute macro generates those functions), or to call
    // the default implementations in std (`__rdl_alloc` etc. in `library/std/src/alloc.rs`)
    // otherwise.
    #[rustc_allocator]
    #[rustc_nounwind]
    #[rustc_std_internal_symbol]
    fn __rust_alloc(size: usize, align: usize) -> *mut u8;
    #[rustc_deallocator]
    #[rustc_nounwind]
    #[rustc_std_internal_symbol]
    fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);
    #[rustc_reallocator]
    #[rustc_nounwind]
    #[rustc_std_internal_symbol]
    fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8;
    #[rustc_allocator_zeroed]
    #[rustc_nounwind]
    #[rustc_std_internal_symbol]
    fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8;

    #[rustc_nounwind]
    #[rustc_std_internal_symbol]
    fn __rust_no_alloc_shim_is_unstable_v2();
}

/// The global memory allocator.
///
/// This type implements the [`Allocator`] trait by forwarding calls
/// to the allocator registered with the `#[global_allocator]` attribute
/// if there is one, or the `std` crate’s default.
///
/// Note: while this type is unstable, the functionality it provides can be
/// accessed through the [free functions in `alloc`](self#functions).
#[unstable(feature = "allocator_api", issue = "32838")]
#[derive(Copy, Clone, Default, Debug)]
// the compiler needs to know when a Box uses the global allocator vs a custom one
#[lang = "global_alloc_ty"]
pub struct Global;

/// Allocates memory with the global allocator.
///
/// This function forwards calls to the [`GlobalAlloc::alloc`] method
/// of the allocator registered with the `#[global_allocator]` attribute
/// if there is one, or the `std` crate’s default.
///
/// This function is expected to be deprecated in favor of the `allocate` method
/// of the [`Global`] type when it and the [`Allocator`] trait become stable.
///
/// # Safety
///
/// See [`GlobalAlloc::alloc`].
///
/// # Examples
///
/// ```
/// use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
///
/// unsafe {
///     let layout = Layout::new::<u16>();
///     let ptr = alloc(layout);
///     if ptr.is_null() {
///         handle_alloc_error(layout);
///     }
///
///     *(ptr as *mut u16) = 42;
///     assert_eq!(*(ptr as *mut u16), 42);
///
///     dealloc(ptr, layout);
/// }
/// ```
#[stable(feature = "global_alloc", since = "1.28.0")]
#[must_use = "losing the pointer will leak memory"]
#[inline]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    unsafe {
        // Make sure we don't accidentally allow omitting the allocator shim in
        // stable code until it is actually stabilized.
        __rust_no_alloc_shim_is_unstable_v2();

        __rust_alloc(layout.size(), layout.align())
    }
}

/// Deallocates memory with the global allocator.
///
/// This function forwards calls to the [`GlobalAlloc::dealloc`] method
/// of the allocator registered with the `#[global_allocator]` attribute
/// if there is one, or the `std` crate’s default.
///
/// This function is expected to be deprecated in favor of the `deallocate` method
/// of the [`Global`] type when it and the [`Allocator`] trait become stable.
///
/// # Safety
///
/// See [`GlobalAlloc::dealloc`].
#[stable(feature = "global_alloc", since = "1.28.0")]
#[inline]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe { __rust_dealloc(ptr, layout.size(), layout.align()) }
}

/// Reallocates memory with the global allocator.
///
/// This function forwards calls to the [`GlobalAlloc::realloc`] method
/// of the allocator registered with the `#[global_allocator]` attribute
/// if there is one, or the `std` crate’s default.
///
/// This function is expected to be deprecated in favor of the `grow` and `shrink` methods
/// of the [`Global`] type when it and the [`Allocator`] trait become stable.
///
/// # Safety
///
/// See [`GlobalAlloc::realloc`].
#[stable(feature = "global_alloc", since = "1.28.0")]
#[must_use = "losing the pointer will leak memory"]
#[inline]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    unsafe { __rust_realloc(ptr, layout.size(), layout.align(), new_size) }
}

/// Allocates zero-initialized memory with the global allocator.
///
/// This function forwards calls to the [`GlobalAlloc::alloc_zeroed`] method
/// of the allocator registered with the `#[global_allocator]` attribute
/// if there is one, or the `std` crate’s default.
///
/// This function is expected to be deprecated in favor of the `allocate_zeroed` method
/// of the [`Global`] type when it and the [`Allocator`] trait become stable.
///
/// # Safety
///
/// See [`GlobalAlloc::alloc_zeroed`].
///
/// # Examples
///
/// ```
/// use std::alloc::{alloc_zeroed, dealloc, handle_alloc_error, Layout};
///
/// unsafe {
///     let layout = Layout::new::<u16>();
///     let ptr = alloc_zeroed(layout);
///     if ptr.is_null() {
///         handle_alloc_error(layout);
///     }
///
///     assert_eq!(*(ptr as *mut u16), 0);
///
///     dealloc(ptr, layout);
/// }
/// ```
#[stable(feature = "global_alloc", since = "1.28.0")]
#[must_use = "losing the pointer will leak memory"]
#[inline]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    unsafe {
        // Make sure we don't accidentally allow omitting the allocator shim in
        // stable code until it is actually stabilized.
        __rust_no_alloc_shim_is_unstable_v2();

        __rust_alloc_zeroed(layout.size(), layout.align())
    }
}

impl Global {
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    fn alloc_impl(&self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError> {
        match layout.size() {
            0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
            // SAFETY: `layout` is non-zero in size,
            size => unsafe {
                let raw_ptr = if zeroed { alloc_zeroed(layout) } else { alloc(layout) };
                let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                Ok(NonNull::slice_from_raw_parts(ptr, size))
            },
        }
    }

    // SAFETY: Same as `Allocator::grow`
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    unsafe fn grow_impl(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
        zeroed: bool,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() >= old_layout.size(),
            "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
        );

        match old_layout.size() {
            0 => self.alloc_impl(new_layout, zeroed),

            // SAFETY: `new_size` is non-zero as `old_size` is greater than or equal to `new_size`
            // as required by safety conditions. Other conditions must be upheld by the caller
            old_size if old_layout.align() == new_layout.align() => unsafe {
                let new_size = new_layout.size();

                // `realloc` probably checks for `new_size >= old_layout.size()` or something similar.
                hint::assert_unchecked(new_size >= old_layout.size());

                let raw_ptr = realloc(ptr.as_ptr(), old_layout, new_size);
                let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                if zeroed {
                    raw_ptr.add(old_size).write_bytes(0, new_size - old_size);
                }
                Ok(NonNull::slice_from_raw_parts(ptr, new_size))
            },

            // SAFETY: because `new_layout.size()` must be greater than or equal to `old_size`,
            // both the old and new memory allocation are valid for reads and writes for `old_size`
            // bytes. Also, because the old allocation wasn't yet deallocated, it cannot overlap
            // `new_ptr`. Thus, the call to `copy_nonoverlapping` is safe. The safety contract
            // for `dealloc` must be upheld by the caller.
            old_size => unsafe {
                let new_ptr = self.alloc_impl(new_layout, zeroed)?;
                ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), old_size);
                self.deallocate(ptr, old_layout);
                Ok(new_ptr)
            },
        }
    }
}

#[unstable(feature = "allocator_api", issue = "32838")]
unsafe impl Allocator for Global {
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.alloc_impl(layout, false)
    }

    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.alloc_impl(layout, true)
    }

    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        if layout.size() != 0 {
            // SAFETY:
            // * We have checked that `layout` is non-zero in size.
            // * The caller is obligated to provide a layout that "fits", and in this case,
            //   "fit" always means a layout that is equal to the original, because our
            //   `allocate()`, `grow()`, and `shrink()` implementations never returns a larger
            //   allocation than requested.
            // * Other conditions must be upheld by the caller, as per `Allocator::deallocate()`'s
            //   safety documentation.
            unsafe { dealloc(ptr.as_ptr(), layout) }
        }
    }

    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        // SAFETY: all conditions must be upheld by the caller
        unsafe { self.grow_impl(ptr, old_layout, new_layout, false) }
    }

    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        // SAFETY: all conditions must be upheld by the caller
        unsafe { self.grow_impl(ptr, old_layout, new_layout, true) }
    }

    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() <= old_layout.size(),
            "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
        );

        match new_layout.size() {
            // SAFETY: conditions must be upheld by the caller
            0 => unsafe {
                self.deallocate(ptr, old_layout);
                Ok(NonNull::slice_from_raw_parts(new_layout.dangling(), 0))
            },

            // SAFETY: `new_size` is non-zero. Other conditions must be upheld by the caller
            new_size if old_layout.align() == new_layout.align() => unsafe {
                // `realloc` probably checks for `new_size <= old_layout.size()` or something similar.
                hint::assert_unchecked(new_size <= old_layout.size());

                let raw_ptr = realloc(ptr.as_ptr(), old_layout, new_size);
                let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                Ok(NonNull::slice_from_raw_parts(ptr, new_size))
            },

            // SAFETY: because `new_size` must be smaller than or equal to `old_layout.size()`,
            // both the old and new memory allocation are valid for reads and writes for `new_size`
            // bytes. Also, because the old allocation wasn't yet deallocated, it cannot overlap
            // `new_ptr`. Thus, the call to `copy_nonoverlapping` is safe. The safety contract
            // for `dealloc` must be upheld by the caller.
            new_size => unsafe {
                let new_ptr = self.allocate(new_layout)?;
                ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), new_size);
                self.deallocate(ptr, old_layout);
                Ok(new_ptr)
            },
        }
    }
}

/// The allocator for `Box`.
#[cfg(not(no_global_oom_handling))]
#[lang = "exchange_malloc"]
#[inline]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
unsafe fn exchange_malloc(size: usize, align: usize) -> *mut u8 {
    let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
    match Global.allocate(layout) {
        Ok(ptr) => ptr.as_mut_ptr(),
        Err(_) => handle_alloc_error(layout),
    }
}

// # Allocation error handler

#[cfg(not(no_global_oom_handling))]
unsafe extern "Rust" {
    // This is the magic symbol to call the global alloc error handler. rustc generates
    // it to call `__rg_oom` if there is a `#[alloc_error_handler]`, or to call the
    // default implementations below (`__rdl_oom`) otherwise.
    #[rustc_std_internal_symbol]
    fn __rust_alloc_error_handler(size: usize, align: usize) -> !;
}

/// Signals a memory allocation error.
///
/// Callers of memory allocation APIs wishing to cease execution
/// in response to an allocation error are encouraged to call this function,
/// rather than directly invoking [`panic!`] or similar.
///
/// This function is guaranteed to diverge (not return normally with a value), but depending on
/// global configuration, it may either panic (resulting in unwinding or aborting as per
/// configuration for all panics), or abort the process (with no unwinding).
///
/// The default behavior is:
///
///  * If the binary links against `std` (typically the case), then
///   print a message to standard error and abort the process.
///   This behavior can be replaced with [`set_alloc_error_hook`] and [`take_alloc_error_hook`].
///   Future versions of Rust may panic by default instead.
///
/// * If the binary does not link against `std` (all of its crates are marked
///   [`#[no_std]`][no_std]), then call [`panic!`] with a message.
///   [The panic handler] applies as to any panic.
///
/// [`set_alloc_error_hook`]: ../../std/alloc/fn.set_alloc_error_hook.html
/// [`take_alloc_error_hook`]: ../../std/alloc/fn.take_alloc_error_hook.html
/// [The panic handler]: https://doc.rust-lang.org/reference/runtime.html#the-panic_handler-attribute
/// [no_std]: https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute
#[stable(feature = "global_alloc", since = "1.28.0")]
#[rustc_const_unstable(feature = "const_alloc_error", issue = "92523")]
#[cfg(not(no_global_oom_handling))]
#[cold]
#[optimize(size)]
pub const fn handle_alloc_error(layout: Layout) -> ! {
    const fn ct_error(_: Layout) -> ! {
        panic!("allocation failed");
    }

    #[inline]
    fn rt_error(layout: Layout) -> ! {
        unsafe {
            __rust_alloc_error_handler(layout.size(), layout.align());
        }
    }

    #[cfg(not(feature = "panic_immediate_abort"))]
    {
        core::intrinsics::const_eval_select((layout,), ct_error, rt_error)
    }

    #[cfg(feature = "panic_immediate_abort")]
    ct_error(layout)
}

#[cfg(not(no_global_oom_handling))]
#[doc(hidden)]
#[allow(unused_attributes)]
#[unstable(feature = "alloc_internals", issue = "none")]
pub mod __alloc_error_handler {
    // called via generated `__rust_alloc_error_handler` if there is no
    // `#[alloc_error_handler]`.
    #[rustc_std_internal_symbol]
    pub unsafe fn __rdl_oom(size: usize, _align: usize) -> ! {
        unsafe extern "Rust" {
            // This symbol is emitted by rustc next to __rust_alloc_error_handler.
            // Its value depends on the -Zoom={panic,abort} compiler option.
            #[rustc_std_internal_symbol]
            fn __rust_alloc_error_handler_should_panic_v2() -> u8;
        }

        if unsafe { __rust_alloc_error_handler_should_panic_v2() != 0 } {
            panic!("memory allocation of {size} bytes failed")
        } else {
            core::panicking::panic_nounwind_fmt(
                format_args!("memory allocation of {size} bytes failed"),
                /* force_no_backtrace */ false,
            )
        }
    }
} } }
