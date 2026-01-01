/* FP:module.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_module_UNPARSEABLE_0001
/* FP:module.rs-0002 */ use std::iter::once;
/* FP:module.rs-0003 */ use std::path::{self, Path, PathBuf};
/* FP:module.rs-0004 */ 
/* FP:module.rs-0005 */ use crate::rustc_complete::{AttrVec, Attribute, Inline, Item, ModSpans};
/* FP:module.rs-0006 */ use rustc_attr_parsing::validate_attr;
/* FP:module.rs-0007 */ use crate::rustc_complete::{Diag, ErrorGuaranteed};
/* FP:module.rs-0008 */ use crate::rustc_parse::lexer::StripTokens;
/* FP:module.rs-0009 */ use crate::rustc_parse::{exp, new_parser_from_file, unwrap_or_emit_fatal};
/* FP:module.rs-0010 */ use crate::rustc_complete::Session;
/* FP:module.rs-0011 */ use crate::rustc_complete::parse::ParseSess;
/* FP:module.rs-0012 */ use crate::rustc_complete::{Ident, Span, sym};
/* FP:module.rs-0013 */ use thin_vec::ThinVec;
/* FP:module.rs-0014 */ 
/* FP:module.rs-0015 */ use crate::base::ModuleData;
/* FP:module.rs-0016 */ use crate::errors::{
/* FP:module.rs-0017 */     ModuleCircular, ModuleFileNotFound, ModuleInBlock, ModuleInBlockName, ModuleMultipleCandidates,
/* FP:module.rs-0018 */ };
/* FP:module.rs-0019 */ 
/* FP:module.rs-0020 */ #[derive(Copy, Clone)]
/* FP:module.rs-0021 */ pub enum DirOwnership {
/* FP:module.rs-0022 */     Owned {
/* FP:module.rs-0023 */         // None if `mod.rs`, `Some("foo")` if we're in `foo.rs`.
/* FP:module.rs-0024 */         relative: Option<Ident>,
/* FP:module.rs-0025 */     },
/* FP:module.rs-0026 */     UnownedViaBlock,
/* FP:module.rs-0027 */ }
/* FP:module.rs-0028 */ 
/* FP:module.rs-0029 */ // Public for rustfmt usage.
/* FP:module.rs-0030 */ pub struct ModulePathSuccess {
/* FP:module.rs-0031 */     pub file_path: PathBuf,
/* FP:module.rs-0032 */     pub dir_ownership: DirOwnership,
/* FP:module.rs-0033 */ }
/* FP:module.rs-0034 */ 
/* FP:module.rs-0035 */ pub(crate) struct ParsedExternalMod {
/* FP:module.rs-0036 */     pub items: ThinVec<Box<Item>>,
/* FP:module.rs-0037 */     pub spans: ModSpans,
/* FP:module.rs-0038 */     pub file_path: PathBuf,
/* FP:module.rs-0039 */     pub dir_path: PathBuf,
/* FP:module.rs-0040 */     pub dir_ownership: DirOwnership,
/* FP:module.rs-0041 */     pub had_parse_error: Result<(), ErrorGuaranteed>,
/* FP:module.rs-0042 */ }
/* FP:module.rs-0043 */ 
/* FP:module.rs-0044 */ pub enum ModError<'a> {
/* FP:module.rs-0045 */     CircularInclusion(Vec<PathBuf>),
/* FP:module.rs-0046 */     ModInBlock(Option<Ident>),
/* FP:module.rs-0047 */     FileNotFound(Ident, PathBuf, PathBuf),
/* FP:module.rs-0048 */     MultipleCandidates(Ident, PathBuf, PathBuf),
/* FP:module.rs-0049 */     ParserError(Diag<'a>),
/* FP:module.rs-0050 */ }
/* FP:module.rs-0051 */ 
/* FP:module.rs-0052 */ pub(crate) fn parse_external_mod(
/* FP:module.rs-0053 */     sess: &Session,
/* FP:module.rs-0054 */     ident: Ident,
/* FP:module.rs-0055 */     span: Span, // The span to blame on errors.
/* FP:module.rs-0056 */     module: &ModuleData,
/* FP:module.rs-0057 */     mut dir_ownership: DirOwnership,
/* FP:module.rs-0058 */     attrs: &mut AttrVec,
/* FP:module.rs-0059 */ ) -> ParsedExternalMod {
/* FP:module.rs-0060 */     // We bail on the first error, but that error does not cause a fatal error... (1)
/* FP:module.rs-0061 */     let result: Result<_, ModError<'_>> = try {
/* FP:module.rs-0062 */         // Extract the file path and the new ownership.
/* FP:module.rs-0063 */         let mp = mod_file_path(sess, ident, attrs, &module.dir_path, dir_ownership)?;
/* FP:module.rs-0064 */         dir_ownership = mp.dir_ownership;
/* FP:module.rs-0065 */ 
/* FP:module.rs-0066 */         // Ensure file paths are acyclic.
/* FP:module.rs-0067 */         if let Some(pos) = module.file_path_stack.iter().position(|p| p == &mp.file_path) {
/* FP:module.rs-0068 */             do yeet ModError::CircularInclusion(module.file_path_stack[pos..].to_vec());
/* FP:module.rs-0069 */         }
/* FP:module.rs-0070 */ 
/* FP:module.rs-0071 */         // Actually parse the external file as a module.
/* FP:module.rs-0072 */         let mut parser = unwrap_or_emit_fatal(new_parser_from_file(
/* FP:module.rs-0073 */             &sess.psess,
/* FP:module.rs-0074 */             &mp.file_path,
/* FP:module.rs-0075 */             StripTokens::ShebangAndFrontmatter,
/* FP:module.rs-0076 */             Some(span),
/* FP:module.rs-0077 */         ));
/* FP:module.rs-0078 */         let (inner_attrs, items, inner_span) =
/* FP:module.rs-0079 */             parser.parse_mod(exp!(Eof)).map_err(|err| ModError::ParserError(err))?;
/* FP:module.rs-0080 */         attrs.extend(inner_attrs);
/* FP:module.rs-0081 */         (items, inner_span, mp.file_path)
/* FP:module.rs-0082 */     };
/* FP:module.rs-0083 */ 
/* FP:module.rs-0084 */     // (1) ...instead, we return a dummy module.
/* FP:module.rs-0085 */     let ((items, spans, file_path), had_parse_error) = match result {
/* FP:module.rs-0086 */         Err(err) => (Default::default(), Err(err.report(sess, span))),
/* FP:module.rs-0087 */         Ok(result) => (result, Ok(())),
/* FP:module.rs-0088 */     };
/* FP:module.rs-0089 */ 
/* FP:module.rs-0090 */     // Extract the directory path for submodules of the module.
/* FP:module.rs-0091 */     let dir_path = file_path.parent().unwrap_or(&file_path).to_owned();
/* FP:module.rs-0092 */ 
/* FP:module.rs-0093 */     ParsedExternalMod { items, spans, file_path, dir_path, dir_ownership, had_parse_error }
/* FP:module.rs-0094 */ }
/* FP:module.rs-0095 */ 
/* FP:module.rs-0096 */ pub(crate) fn mod_dir_path(
/* FP:module.rs-0097 */     sess: &Session,
/* FP:module.rs-0098 */     ident: Ident,
/* FP:module.rs-0099 */     attrs: &[Attribute],
/* FP:module.rs-0100 */     module: &ModuleData,
/* FP:module.rs-0101 */     mut dir_ownership: DirOwnership,
/* FP:module.rs-0102 */     inline: Inline,
/* FP:module.rs-0103 */ ) -> (PathBuf, DirOwnership) {
/* FP:module.rs-0104 */     match inline {
/* FP:module.rs-0105 */         Inline::Yes
/* FP:module.rs-0106 */             if let Some(file_path) = mod_file_path_from_attr(sess, attrs, &module.dir_path) =>
/* FP:module.rs-0107 */         {
/* FP:module.rs-0108 */             // For inline modules file path from `#[path]` is actually the directory path
/* FP:module.rs-0109 */             // for historical reasons, so we don't pop the last segment here.
/* FP:module.rs-0110 */             (file_path, DirOwnership::Owned { relative: None })
/* FP:module.rs-0111 */         }
/* FP:module.rs-0112 */         Inline::Yes => {
/* FP:module.rs-0113 */             // We have to push on the current module name in the case of relative
/* FP:module.rs-0114 */             // paths in order to ensure that any additional module paths from inline
/* FP:module.rs-0115 */             // `mod x { ... }` come after the relative extension.
/* FP:module.rs-0116 */             //
/* FP:module.rs-0117 */             // For example, a `mod z { ... }` inside `x/y.rs` should set the current
/* FP:module.rs-0118 */             // directory path to `/x/y/z`, not `/x/z` with a relative offset of `y`.
/* FP:module.rs-0119 */             let mut dir_path = module.dir_path.clone();
/* FP:module.rs-0120 */             if let DirOwnership::Owned { relative } = &mut dir_ownership {
/* FP:module.rs-0121 */                 if let Some(ident) = relative.take() {
/* FP:module.rs-0122 */                     // Remove the relative offset.
/* FP:module.rs-0123 */                     dir_path.push(ident.as_str());
/* FP:module.rs-0124 */                 }
/* FP:module.rs-0125 */             }
/* FP:module.rs-0126 */             dir_path.push(ident.as_str());
/* FP:module.rs-0127 */ 
/* FP:module.rs-0128 */             (dir_path, dir_ownership)
/* FP:module.rs-0129 */         }
/* FP:module.rs-0130 */         Inline::No { .. } => {
/* FP:module.rs-0131 */             // FIXME: This is a subset of `parse_external_mod` without actual parsing,
/* FP:module.rs-0132 */             // check whether the logic for unloaded, loaded and inline modules can be unified.
/* FP:module.rs-0133 */             let file_path = mod_file_path(sess, ident, attrs, &module.dir_path, dir_ownership)
/* FP:module.rs-0134 */                 .map(|mp| {
/* FP:module.rs-0135 */                     dir_ownership = mp.dir_ownership;
/* FP:module.rs-0136 */                     mp.file_path
/* FP:module.rs-0137 */                 })
/* FP:module.rs-0138 */                 .unwrap_or_default();
/* FP:module.rs-0139 */ 
/* FP:module.rs-0140 */             // Extract the directory path for submodules of the module.
/* FP:module.rs-0141 */             let dir_path = file_path.parent().unwrap_or(&file_path).to_owned();
/* FP:module.rs-0142 */ 
/* FP:module.rs-0143 */             (dir_path, dir_ownership)
/* FP:module.rs-0144 */         }
/* FP:module.rs-0145 */     }
/* FP:module.rs-0146 */ }
/* FP:module.rs-0147 */ 
/* FP:module.rs-0148 */ fn mod_file_path<'a>(
/* FP:module.rs-0149 */     sess: &'a Session,
/* FP:module.rs-0150 */     ident: Ident,
/* FP:module.rs-0151 */     attrs: &[Attribute],
/* FP:module.rs-0152 */     dir_path: &Path,
/* FP:module.rs-0153 */     dir_ownership: DirOwnership,
/* FP:module.rs-0154 */ ) -> Result<ModulePathSuccess, ModError<'a>> {
/* FP:module.rs-0155 */     if let Some(file_path) = mod_file_path_from_attr(sess, attrs, dir_path) {
/* FP:module.rs-0156 */         // All `#[path]` files are treated as though they are a `mod.rs` file.
/* FP:module.rs-0158 */         // files are siblings,
/* FP:module.rs-0159 */         //
/* FP:module.rs-0160 */         // Note that this will produce weirdness when a file named `foo.rs` is
/* FP:module.rs-0162 */         // If you encounter this, it's your own darn fault :P
/* FP:module.rs-0163 */         let dir_ownership = DirOwnership::Owned { relative: None };
/* FP:module.rs-0164 */         return Ok(ModulePathSuccess { file_path, dir_ownership });
/* FP:module.rs-0165 */     }
/* FP:module.rs-0166 */ 
/* FP:module.rs-0167 */     let relative = match dir_ownership {
/* FP:module.rs-0168 */         DirOwnership::Owned { relative } => relative,
/* FP:module.rs-0169 */         DirOwnership::UnownedViaBlock => None,
/* FP:module.rs-0170 */     };
/* FP:module.rs-0171 */     let result = default_submod_path(&sess.psess, ident, relative, dir_path);
/* FP:module.rs-0172 */     match dir_ownership {
/* FP:module.rs-0173 */         DirOwnership::Owned { .. } => result,
/* FP:module.rs-0174 */         DirOwnership::UnownedViaBlock => Err(ModError::ModInBlock(match result {
/* FP:module.rs-0175 */             Ok(_) | Err(ModError::MultipleCandidates(..)) => Some(ident),
/* FP:module.rs-0176 */             _ => None,
/* FP:module.rs-0177 */         })),
/* FP:module.rs-0178 */     }
/* FP:module.rs-0179 */ }
/* FP:module.rs-0180 */ 
/* FP:module.rs-0181 */ /// Derive a submodule path from the first found `#[path = "path_string"]`.
/* FP:module.rs-0182 */ /// The provided `dir_path` is joined with the `path_string`.
/* FP:module.rs-0183 */ pub(crate) fn mod_file_path_from_attr(
/* FP:module.rs-0184 */     sess: &Session,
/* FP:module.rs-0185 */     attrs: &[Attribute],
/* FP:module.rs-0186 */     dir_path: &Path,
/* FP:module.rs-0187 */ ) -> Option<PathBuf> {
/* FP:module.rs-0188 */     // Extract path string from first `#[path = "path_string"]` attribute.
/* FP:module.rs-0189 */     let first_path = attrs.iter().find(|at| at.has_name(sym::path))?;
/* FP:module.rs-0190 */     let Some(path_sym) = first_path.value_str() else {
/* FP:module.rs-0191 */         // This check is here mainly to catch attempting to use a macro,
/* FP:module.rs-0192 */         // such as `#[path = concat!(...)]`. This isn't supported because
/* FP:module.rs-0193 */         // otherwise the `InvocationCollector` would need to defer loading
/* FP:module.rs-0194 */         // a module until the `#[path]` attribute was expanded, and it
/* FP:module.rs-0195 */         // doesn't support that (and would likely add a bit of complexity).
/* FP:module.rs-0196 */         // Usually bad forms are checked during semantic analysis via
/* FP:module.rs-0197 */         // `TyCtxt::check_mod_attrs`), but by the time that runs the macro
/* FP:module.rs-0198 */         // is expanded, and it doesn't give an error.
/* FP:module.rs-0199 */         validate_attr::emit_fatal_malformed_builtin_attribute(&sess.psess, first_path, sym::path);
/* FP:module.rs-0200 */     };
/* FP:module.rs-0201 */ 
/* FP:module.rs-0202 */     let path_str = path_sym.as_str();
/* FP:module.rs-0203 */ 
/* FP:module.rs-0204 */     // On windows, the base path might have the form
/* FP:module.rs-0205 */     // `\\?\foo\bar` in which case it does not tolerate
/* FP:module.rs-0206 */     // mixed `/` and `\` separators, so canonicalize
/* FP:module.rs-0207 */     // `/` to `\`.
/* FP:module.rs-0208 */     #[cfg(windows)]
/* FP:module.rs-0209 */     let path_str = path_str.replace("/", "\\");
/* FP:module.rs-0210 */ 
/* FP:module.rs-0211 */     Some(dir_path.join(path_str))
/* FP:module.rs-0212 */ }
/* FP:module.rs-0213 */ 
/* FP:module.rs-0214 */ /// Returns a path to a module.
/* FP:module.rs-0215 */ // Public for rustfmt usage.
/* FP:module.rs-0216 */ pub fn default_submod_path<'a>(
/* FP:module.rs-0217 */     psess: &'a ParseSess,
/* FP:module.rs-0218 */     ident: Ident,
/* FP:module.rs-0219 */     relative: Option<Ident>,
/* FP:module.rs-0220 */     dir_path: &Path,
/* FP:module.rs-0221 */ ) -> Result<ModulePathSuccess, ModError<'a>> {
/* FP:module.rs-0222 */     // If we're in a foo.rs file instead of a mod.rs file,
/* FP:module.rs-0223 */     // we need to look for submodules in
/* FP:module.rs-0224 */     // `./foo/<ident>.rs` and `./foo/<ident>/mod.rs` rather than
/* FP:module.rs-0225 */     // `./<ident>.rs` and `./<ident>/mod.rs`.
/* FP:module.rs-0226 */     let relative_prefix_string;
/* FP:module.rs-0227 */     let relative_prefix = if let Some(ident) = relative {
/* FP:module.rs-0228 */         relative_prefix_string = format!("{}{}", ident.name, path::MAIN_SEPARATOR);
/* FP:module.rs-0229 */         &relative_prefix_string
/* FP:module.rs-0230 */     } else {
/* FP:module.rs-0231 */         ""
/* FP:module.rs-0232 */     };
/* FP:module.rs-0233 */ 
/* FP:module.rs-0234 */     let default_path_str = format!("{}{}.rs", relative_prefix, ident.name);
/* FP:module.rs-0235 */     let secondary_path_str =
/* FP:module.rs-0236 */         format!("{}{}{}mod.rs", relative_prefix, ident.name, path::MAIN_SEPARATOR);
/* FP:module.rs-0237 */     let default_path = dir_path.join(&default_path_str);
/* FP:module.rs-0238 */     let secondary_path = dir_path.join(&secondary_path_str);
/* FP:module.rs-0239 */     let default_exists = psess.source_map().file_exists(&default_path);
/* FP:module.rs-0240 */     let secondary_exists = psess.source_map().file_exists(&secondary_path);
/* FP:module.rs-0241 */ 
/* FP:module.rs-0242 */     match (default_exists, secondary_exists) {
/* FP:module.rs-0243 */         (true, false) => Ok(ModulePathSuccess {
/* FP:module.rs-0244 */             file_path: default_path,
/* FP:module.rs-0245 */             dir_ownership: DirOwnership::Owned { relative: Some(ident) },
/* FP:module.rs-0246 */         }),
/* FP:module.rs-0247 */         (false, true) => Ok(ModulePathSuccess {
/* FP:module.rs-0248 */             file_path: secondary_path,
/* FP:module.rs-0249 */             dir_ownership: DirOwnership::Owned { relative: None },
/* FP:module.rs-0250 */         }),
/* FP:module.rs-0251 */         (false, false) => Err(ModError::FileNotFound(ident, default_path, secondary_path)),
/* FP:module.rs-0252 */         (true, true) => Err(ModError::MultipleCandidates(ident, default_path, secondary_path)),
/* FP:module.rs-0253 */     }
/* FP:module.rs-0254 */ }
/* FP:module.rs-0255 */ 
/* FP:module.rs-0256 */ impl ModError<'_> {
/* FP:module.rs-0257 */     fn report(self, sess: &Session, span: Span) -> ErrorGuaranteed {
/* FP:module.rs-0258 */         match self {
/* FP:module.rs-0259 */             ModError::CircularInclusion(file_paths) => {
/* FP:module.rs-0260 */                 let path_to_string = |path: &PathBuf| path.display().to_string();
/* FP:module.rs-0261 */ 
/* FP:module.rs-0262 */                 let paths = file_paths
/* FP:module.rs-0263 */                     .iter()
/* FP:module.rs-0264 */                     .map(path_to_string)
/* FP:module.rs-0265 */                     .chain(once(path_to_string(&file_paths[0])))
/* FP:module.rs-0266 */                     .collect::<Vec<_>>();
/* FP:module.rs-0267 */ 
/* FP:module.rs-0268 */                 let modules = paths.join(" -> ");
/* FP:module.rs-0269 */ 
/* FP:module.rs-0270 */                 sess.dcx().emit_err(ModuleCircular { span, modules })
/* FP:module.rs-0271 */             }
/* FP:module.rs-0272 */             ModError::ModInBlock(ident) => sess.dcx().emit_err(ModuleInBlock {
/* FP:module.rs-0273 */                 span,
/* FP:module.rs-0274 */                 name: ident.map(|name| ModuleInBlockName { span, name }),
/* FP:module.rs-0275 */             }),
/* FP:module.rs-0276 */             ModError::FileNotFound(name, default_path, secondary_path) => {
/* FP:module.rs-0277 */                 sess.dcx().emit_err(ModuleFileNotFound {
/* FP:module.rs-0278 */                     span,
/* FP:module.rs-0279 */                     name,
/* FP:module.rs-0280 */                     default_path: default_path.display().to_string(),
/* FP:module.rs-0281 */                     secondary_path: secondary_path.display().to_string(),
/* FP:module.rs-0282 */                 })
/* FP:module.rs-0283 */             }
/* FP:module.rs-0284 */             ModError::MultipleCandidates(name, default_path, secondary_path) => {
/* FP:module.rs-0285 */                 sess.dcx().emit_err(ModuleMultipleCandidates {
/* FP:module.rs-0286 */                     span,
/* FP:module.rs-0287 */                     name,
/* FP:module.rs-0288 */                     default_path: default_path.display().to_string(),
/* FP:module.rs-0289 */                     secondary_path: secondary_path.display().to_string(),
/* FP:module.rs-0290 */                 })
/* FP:module.rs-0291 */             }
/* FP:module.rs-0292 */             ModError::ParserError(err) => err.emit(),
/* FP:module.rs-0293 */         }
/* FP:module.rs-0294 */     }
/* FP:module.rs-0295 */ }