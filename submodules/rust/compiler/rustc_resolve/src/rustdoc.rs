mkuse!{use std :: sync :: Mutex ;}
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: sync :: LazyLock ;}
mkitem!{static USE_MATRIX : LazyLock < Mutex < HashMap < String , Vec < String > > > > = LazyLock :: new (| | Mutex :: new (HashMap :: new ())) ;}

macro_rules! get_use_matrix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_use_matrix in module {}", module_path!());
    };
}

mkfn!{
    get_use_matrix_introspect!();
    pub fn get_use_matrix () -> HashMap < String , Vec < String > > { USE_MATRIX . lock () . unwrap () . clone () }
}
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { { use std :: fs :: OpenOptions ; use std :: io :: Write ; let message = format ! ($ ($ arg) *) ; if let Ok (mut file) = OpenOptions :: new () . create (true) . append (true) . open ("macro_report.txt") { let _ = writeln ! (file , "{}" , message) ; } } } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (#[$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (#[$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{#[macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{#[macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { emit_message ! ("USE|{}|{}" , module_path ! () , stringify ! ($ use_stmt)) ; $ use_stmt } ; }}
mkitem!{macro_rules ! mkstruct { ($ struct_def : item) => { $ struct_def } ; }}
mkitem!{macro_rules ! mkenum { ($ enum_def : item) => { $ enum_def } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { stringify ! ($ name) } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { "processed file" } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { "processed_path" } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { vec ! [] } ; }}
mkmod!{rustc_complete, { 
                getname!(rustc_complete);
                getsrc!(rustc_complete);
                getpath!(rustc_complete);
                get_deps!(rustc_complete);
                get_crates!(rustc_complete);
                mkinclude!(rustc_complete);
                mkmod!{emitter, { 
                getname!(emitter);
                getsrc!(emitter);
                getpath!(emitter);
                get_deps!(emitter);
                get_crates!(emitter);
                mkinclude!(emitter);
                
macro_rules! stderr_destination_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stderr_destination in module {}", module_path!());
    };
}

mkfn!{
    stderr_destination_introspect!();
    pub fn stderr_destination () { }
} 
            }}
mkmod!{registry, { 
                getname!(registry);
                getsrc!(registry);
                getpath!(registry);
                get_deps!(registry);
                get_crates!(registry);
                mkinclude!(registry);
                mkitem!{mkstruct!{pub struct Registry ;}} 
            }}
mkmod!{translation, { 
                getname!(translation);
                getsrc!(translation);
                getpath!(translation);
                get_deps!(translation);
                get_crates!(translation);
                mkinclude!(translation);
                mkitem!{mkstruct!{pub struct Translator ;}} 
            }}
mkitem!{mkstruct!{pub struct ColorConfig ;}}
mkitem!{mkstruct!{pub struct DiagCtxt ;}}
mkitem!{mkstruct!{pub struct ErrCode ;}}
mkitem!{mkstruct!{pub struct FatalError ;}}
mkitem!{mkstruct!{pub struct PResult < T > (pub T) ;}}
mkmod!{markdown, { 
                getname!(markdown);
                getsrc!(markdown);
                getpath!(markdown);
                get_deps!(markdown);
                get_crates!(markdown);
                mkinclude!(markdown);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                mkitem!{mkstruct!{pub struct CG_OPTIONS ;}}
mkitem!{mkstruct!{pub struct CrateType ;}}
mkitem!{mkstruct!{pub struct ErrorOutputType ;}}
mkitem!{mkstruct!{pub struct Input ;}}
mkitem!{mkstruct!{pub struct OptionDesc ;}}
mkitem!{mkstruct!{pub struct OutFileName ;}}
mkitem!{mkstruct!{pub struct OutputType ;}}
mkitem!{mkstruct!{pub struct Sysroot ;}}
mkitem!{mkstruct!{pub struct UnstableOptions ;}}
mkitem!{mkstruct!{pub struct Z_OPTIONS ;}}

macro_rules! nightly_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nightly_options in module {}", module_path!());
    };
}

mkfn!{
    nightly_options_introspect!();
    pub fn nightly_options () { }
}

macro_rules! parse_target_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_target_triple in module {}", module_path!());
    };
}

mkfn!{
    parse_target_triple_introspect!();
    pub fn parse_target_triple () { }
} 
            }}
mkmod!{getopts, { 
                getname!(getopts);
                getsrc!(getopts);
                getpath!(getopts);
                get_deps!(getopts);
                get_crates!(getopts);
                mkinclude!(getopts);
                mkitem!{mkstruct!{pub struct Matches ;}} 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                mkitem!{mkstruct!{pub struct Lint ;}}
mkitem!{mkstruct!{pub struct LintId ;}} 
            }}
mkmod!{output, { 
                getname!(output);
                getsrc!(output);
                getpath!(output);
                get_deps!(output);
                get_crates!(output);
                mkinclude!(output);
                mkitem!{mkstruct!{pub struct CRATE_TYPES ;}}

macro_rules! collect_crate_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_crate_types in module {}", module_path!());
    };
}

mkfn!{
    collect_crate_types_introspect!();
    pub fn collect_crate_types () { }
}

macro_rules! invalid_output_for_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_output_for_target in module {}", module_path!());
    };
}

mkfn!{
    invalid_output_for_target_introspect!();
    pub fn invalid_output_for_target () { }
} 
            }}
mkitem!{mkstruct!{pub struct EarlyDiagCtxt ;}}
mkitem!{mkstruct!{pub struct Session ;}}
mkitem!{mkstruct!{pub struct FileName ;}}
mkmod!{def_id, { 
                getname!(def_id);
                getsrc!(def_id);
                getpath!(def_id);
                get_deps!(def_id);
                get_crates!(def_id);
                mkinclude!(def_id);
                mkitem!{mkstruct!{pub struct LOCAL_CRATE ;}} 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                mkitem!{mkstruct!{pub struct TyCtxt < T > (pub T) ;}} 
            }} 
            }}
mkmod!{session_diagnostics, { 
                getname!(session_diagnostics);
                getsrc!(session_diagnostics);
                getpath!(session_diagnostics);
                get_deps!(session_diagnostics);
                get_crates!(session_diagnostics);
                mkinclude!(session_diagnostics);
                mkitem!{mkstruct!{pub struct CantEmitMIR ;}}
mkitem!{mkstruct!{pub struct RLinkEmptyVersionNumber ;}}
mkitem!{mkstruct!{pub struct RLinkEncodingVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkRustcVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkWrongFileType ;}}
mkitem!{mkstruct!{pub struct RlinkCorruptFile ;}}
mkitem!{mkstruct!{pub struct RlinkNotAFile ;}}
mkitem!{mkstruct!{pub struct RlinkUnableToRead ;}}
mkitem!{mkstruct!{pub struct UnstableFeatureUsage ;}} 
            }}
mkitem!{macro_rules ! do_not_use_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use print") } ; }}
mkitem!{macro_rules ! do_not_use_safe_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use safe_print") } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { pub fn get_module_name () -> &'static str { stringify ! ($ name) } } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { pub fn get_source_info () -> &'static str { concat ! ("Module: " , stringify ! ($ name)) } } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { pub fn get_module_path () -> &'static str { module_path ! () } } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { pub fn get_dependencies () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! get_crates { ($ name : ident) => { pub fn get_required_crates () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! forall_crates { ($ ($ crate_name : ident) ,*) => { $ (extern crate $ crate_name ;) * } ; }}
mkitem!{macro_rules ! emit_extern { ($ crate_name : ident) => { extern crate $ crate_name ; } ; }}
mkitem!{macro_rules ! get_externs { ($ crate_name : ident) => { stringify ! ($ crate_name) } ; }}
mkuse!{use std :: mem ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use itertools :: Itertools ;}
mkuse!{use pulldown_cmark :: { BrokenLink , BrokenLinkCallback , CowStr , Event , LinkType , Options , Parser , Tag , } ;}
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: attr :: AttributeExt ;}
mkuse!{use rustc_ast :: join_path_syms ;}
mkuse!{use rustc_ast :: util :: comments :: beautify_doc_string ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_data_structures :: unord :: UnordSet ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: source_map :: SourceMap ;}
mkuse!{use rustc_span :: { DUMMY_SP , InnerSpan , Span , Symbol , sym } ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use tracing :: { debug , trace } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{mkenum!{#[derive (Clone , Copy , PartialEq , Eq , Debug)] pub enum DocFragmentKind { #[doc = " A doc fragment created from a `///` or `//` doc comment."] SugaredDoc , #[doc = " A doc fragment created from a \"raw\" `#[doc=\"\"]` attribute."] RawDoc , }}}
mkitem!{mkstruct!{#[doc = " A portion of documentation, extracted from a `#[doc]` attribute."] #[doc = ""] #[doc = " Each variant contains the line number within the complete doc-comment where the fragment"] #[doc = " starts, as well as the Span where the corresponding doc comment or attribute is located."] #[doc = ""] #[doc = " Included files are kept separate from inline doc comments so that proper line-number"] #[doc = " information can be given when a doctest fails. Sugared doc comments and \"raw\" doc comments are"] #[doc = " kept separate because of issue #42760."] #[derive (Clone , PartialEq , Eq , Debug)] pub struct DocFragment { pub span : Span , #[doc = " The item this doc-comment came from."] #[doc = " Used to determine the scope in which doc links in this fragment are resolved."] #[doc = " Typically filled for reexport docs when they are merged into the docs of the"] #[doc = " original reexported item."] #[doc = " If the id is not filled, which happens for the original reexported item, then"] #[doc = " it has to be taken from somewhere else during doc link resolution."] pub item_id : Option < DefId > , pub doc : Symbol , pub kind : DocFragmentKind , pub indent : usize , #[doc = " Because we tamper with the spans context, this information cannot be correctly retrieved"] #[doc = " later on. So instead, we compute it and store it here."] pub from_expansion : bool , }}}
mkitem!{mkenum!{#[derive (Clone , Copy , Debug)] pub enum MalformedGenerics { #[doc = " This link has unbalanced angle brackets."] #[doc = ""] #[doc = " For example, `Vec<T` should trigger this, as should `Vec<T>>`."] UnbalancedAngleBrackets , #[doc = " The generics are not attached to a type."] #[doc = ""] #[doc = " For example, `<T>` should trigger this."] #[doc = ""] #[doc = " This is detected by checking if the path is empty after the generics are stripped."] MissingType , #[doc = " The link uses fully-qualified syntax, which is currently unsupported."] #[doc = ""] #[doc = " For example, `<Vec as IntoIterator>::into_iter` should trigger this."] #[doc = ""] #[doc = " This is detected by checking if ` as ` (the keyword `as` with spaces around it) is inside"] #[doc = " angle brackets."] HasFullyQualifiedSyntax , #[doc = " The link has an invalid path separator."] #[doc = ""] #[doc = " For example, `Vec:<T>:new()` should trigger this. Note that `Vec:new()` will **not**"] #[doc = " trigger this because it has no generics and thus [`strip_generics_from_path`] will not be"] #[doc = " called."] #[doc = ""] #[doc = " Note that this will also **not** be triggered if the invalid path separator is inside angle"] #[doc = " brackets because rustdoc mostly ignores what's inside angle brackets (except for"] #[doc = " [`HasFullyQualifiedSyntax`](MalformedGenerics::HasFullyQualifiedSyntax))."] #[doc = ""] #[doc = " This is detected by checking if there is a colon followed by a non-colon in the link."] InvalidPathSeparator , #[doc = " The link has too many angle brackets."] #[doc = ""] #[doc = " For example, `Vec<<T>>` should trigger this."] TooManyAngleBrackets , #[doc = " The link has empty angle brackets."] #[doc = ""] #[doc = " For example, `Vec<>` should trigger this."] EmptyAngleBrackets , }}}

macro_rules! unindent_doc_fragments_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unindent_doc_fragments in module {}", module_path!());
    };
}

mkfn!{
    unindent_doc_fragments_introspect!();
    #[doc = " Removes excess indentation on comments in order for the Markdown"] #[doc = " to be parsed correctly. This is necessary because the convention for"] #[doc = " writing documentation is to provide a space between the /// or // marker"] #[doc = " and the doc text, but Markdown is whitespace-sensitive. For example,"] #[doc = " a block of text with four-space indentation is parsed as a code block,"] #[doc = " so if we didn't unindent comments, these list items"] #[doc = ""] #[doc = " /// A list:"] #[doc = " ///"] #[doc = " ///    - Foo"] #[doc = " ///    - Bar"] #[doc = ""] #[doc = " would be parsed as if they were in a code block, which is likely not what the user intended."] pub fn unindent_doc_fragments (docs : & mut [DocFragment]) { let add = if docs . windows (2) . any (| arr | arr [0] . kind != arr [1] . kind) && docs . iter () . any (| d | d . kind == DocFragmentKind :: SugaredDoc) { 1 } else { 0 } ; let Some (min_indent) = docs . iter () . map (| fragment | { fragment . doc . as_str () . lines () . filter (| line | line . chars () . any (| c | ! c . is_whitespace ())) . map (| line | { let whitespace = line . chars () . take_while (| c | * c == ' ' || * c == '\t') . count () ; whitespace + (if fragment . kind == DocFragmentKind :: SugaredDoc { 0 } else { add }) }) . min () . unwrap_or (usize :: MAX) }) . min () else { return ; } ; for fragment in docs { if fragment . doc == sym :: empty { continue ; } let indent = if fragment . kind != DocFragmentKind :: SugaredDoc && min_indent > 0 { min_indent - add } else { min_indent } ; fragment . indent = indent ; } }
}

macro_rules! add_doc_fragment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_doc_fragment in module {}", module_path!());
    };
}

mkfn!{
    add_doc_fragment_introspect!();
    #[doc = " The goal of this function is to apply the `DocFragment` transformation that is required when"] #[doc = " transforming into the final Markdown, which is applying the computed indent to each line in"] #[doc = " each doc fragment (a `DocFragment` can contain multiple lines in case of `#[doc = \"\"]`)."] #[doc = ""] #[doc = " Note: remove the trailing newline where appropriate"] pub fn add_doc_fragment (out : & mut String , frag : & DocFragment) { if frag . doc == sym :: empty { out . push ('\n') ; return ; } let s = frag . doc . as_str () ; let mut iter = s . lines () ; while let Some (line) = iter . next () { if line . chars () . any (| c | ! c . is_whitespace ()) { assert ! (line . len () >= frag . indent) ; out . push_str (& line [frag . indent ..]) ; } else { out . push_str (line) ; } out . push ('\n') ; } }
}

macro_rules! attrs_to_doc_fragments_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function attrs_to_doc_fragments in module {}", module_path!());
    };
}

mkfn!{
    attrs_to_doc_fragments_introspect!();
    pub fn attrs_to_doc_fragments < 'a , A : AttributeExt + Clone + 'a > (attrs : impl Iterator < Item = (& 'a A , Option < DefId >) > , doc_only : bool ,) -> (Vec < DocFragment > , ThinVec < A >) { let (min_size , max_size) = attrs . size_hint () ; let size_hint = max_size . unwrap_or (min_size) ; let mut doc_fragments = Vec :: with_capacity (size_hint) ; let mut other_attrs = ThinVec :: < A > :: with_capacity (if doc_only { 0 } else { size_hint }) ; for (attr , item_id) in attrs { if let Some ((doc_str , comment_kind)) = attr . doc_str_and_comment_kind () { let doc = beautify_doc_string (doc_str , comment_kind) ; let (span , kind , from_expansion) = if attr . is_doc_comment () { let span = attr . span () ; (span , DocFragmentKind :: SugaredDoc , span . from_expansion ()) } else { let attr_span = attr . span () ; let (span , from_expansion) = match attr . value_span () { Some (sp) => (sp . with_ctxt (attr_span . ctxt ()) , sp . from_expansion ()) , None => (attr_span , attr_span . from_expansion ()) , } ; (span , DocFragmentKind :: RawDoc , from_expansion) } ; let fragment = DocFragment { span , doc , kind , item_id , indent : 0 , from_expansion } ; doc_fragments . push (fragment) ; } else if ! doc_only { other_attrs . push (attr . clone ()) ; } } doc_fragments . shrink_to_fit () ; other_attrs . shrink_to_fit () ; unindent_doc_fragments (& mut doc_fragments) ; (doc_fragments , other_attrs) }
}

macro_rules! prepare_to_doc_link_resolution_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_to_doc_link_resolution in module {}", module_path!());
    };
}

mkfn!{
    prepare_to_doc_link_resolution_introspect!();
    #[doc = " Return the doc-comments on this item, grouped by the module they came from."] #[doc = " The module can be different if this is a re-export with added documentation."] #[doc = ""] #[doc = " The last newline is not trimmed so the produced strings are reusable between"] #[doc = " early and late doc link resolution regardless of their position."] pub fn prepare_to_doc_link_resolution (doc_fragments : & [DocFragment] ,) -> FxIndexMap < Option < DefId > , String > { let mut res = FxIndexMap :: default () ; for fragment in doc_fragments { let out_str = res . entry (fragment . item_id) . or_default () ; add_doc_fragment (out_str , fragment) ; } res }
}

macro_rules! main_body_opts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main_body_opts in module {}", module_path!());
    };
}

mkfn!{
    main_body_opts_introspect!();
    #[doc = " Options for rendering Markdown in the main body of documentation."] pub fn main_body_opts () -> Options { Options :: ENABLE_TABLES | Options :: ENABLE_FOOTNOTES | Options :: ENABLE_STRIKETHROUGH | Options :: ENABLE_TASKLISTS | Options :: ENABLE_SMART_PUNCTUATION }
}

macro_rules! strip_generics_from_path_segment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function strip_generics_from_path_segment in module {}", module_path!());
    };
}

mkfn!{
    strip_generics_from_path_segment_introspect!();
    fn strip_generics_from_path_segment (segment : Vec < char >) -> Result < Symbol , MalformedGenerics > { let mut stripped_segment = String :: new () ; let mut param_depth = 0 ; let mut latest_generics_chunk = String :: new () ; for c in segment { if c == '<' { param_depth += 1 ; latest_generics_chunk . clear () ; } else if c == '>' { param_depth -= 1 ; if latest_generics_chunk . contains (" as ") { return Err (MalformedGenerics :: HasFullyQualifiedSyntax) ; } } else if param_depth == 0 { stripped_segment . push (c) ; } else { latest_generics_chunk . push (c) ; } } if param_depth == 0 { Ok (Symbol :: intern (& stripped_segment)) } else { Err (MalformedGenerics :: UnbalancedAngleBrackets) } }
}

macro_rules! strip_generics_from_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function strip_generics_from_path in module {}", module_path!());
    };
}

mkfn!{
    strip_generics_from_path_introspect!();
    pub fn strip_generics_from_path (path_str : & str) -> Result < Box < str > , MalformedGenerics > { if ! path_str . contains (['<' , '>']) { return Ok (path_str . into ()) ; } let mut stripped_segments = vec ! [] ; let mut path = path_str . chars () . peekable () ; let mut segment = Vec :: new () ; while let Some (chr) = path . next () { match chr { ':' => { if path . next_if_eq (& ':') . is_some () { let stripped_segment = strip_generics_from_path_segment (mem :: take (& mut segment)) ? ; if ! stripped_segment . is_empty () { stripped_segments . push (stripped_segment) ; } } else { return Err (MalformedGenerics :: InvalidPathSeparator) ; } } '<' => { segment . push (chr) ; match path . next () { Some ('<') => { return Err (MalformedGenerics :: TooManyAngleBrackets) ; } Some ('>') => { return Err (MalformedGenerics :: EmptyAngleBrackets) ; } Some (chr) => { segment . push (chr) ; while let Some (chr) = path . next_if (| c | * c != '>') { segment . push (chr) ; } } None => break , } } _ => segment . push (chr) , } trace ! ("raw segment: {:?}" , segment) ; } if ! segment . is_empty () { let stripped_segment = strip_generics_from_path_segment (segment) ? ; if ! stripped_segment . is_empty () { stripped_segments . push (stripped_segment) ; } } debug ! ("path_str: {path_str:?}\nstripped segments: {stripped_segments:?}") ; if ! stripped_segments . is_empty () { let stripped_path = join_path_syms (stripped_segments) ; Ok (stripped_path . into ()) } else { Err (MalformedGenerics :: MissingType) } }
}

macro_rules! inner_docs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inner_docs in module {}", module_path!());
    };
}

mkfn!{
    inner_docs_introspect!();
    #[doc = " Returns whether the first doc-comment is an inner attribute."] #[doc = ""] #[doc = " If there are no doc-comments, return true."] #[doc = " FIXME(#78591): Support both inner and outer attributes on the same item."] pub fn inner_docs (attrs : & [impl AttributeExt]) -> bool { for attr in attrs { if let Some (attr_style) = attr . doc_resolution_scope () { return attr_style == ast :: AttrStyle :: Inner ; } } true }
}

macro_rules! has_primitive_or_keyword_or_attribute_docs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_primitive_or_keyword_or_attribute_docs in module {}", module_path!());
    };
}

mkfn!{
    has_primitive_or_keyword_or_attribute_docs_introspect!();
    #[doc = " Has `#[rustc_doc_primitive]` or `#[doc(keyword)]` or `#[doc(attribute)]`."] pub fn has_primitive_or_keyword_or_attribute_docs (attrs : & [impl AttributeExt]) -> bool { for attr in attrs { if attr . has_name (sym :: rustc_doc_primitive) { return true ; } else if attr . has_name (sym :: doc) && let Some (items) = attr . meta_item_list () { for item in items { if item . has_name (sym :: keyword) || item . has_name (sym :: attribute) { return true ; } } } } false }
}

macro_rules! preprocess_link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function preprocess_link in module {}", module_path!());
    };
}

mkfn!{
    preprocess_link_introspect!();
    #[doc = " Simplified version of the corresponding function in rustdoc."] #[doc = " If the rustdoc version returns a successful result, this function must return the same result."] #[doc = " Otherwise this function may return anything."] fn preprocess_link (link : & str) -> Box < str > { let link = link . replace ('`' , "") ; let link = link . split ('#') . next () . unwrap () ; let link = link . trim () ; let link = link . rsplit ('@') . next () . unwrap () ; let link = link . strip_suffix ("()") . unwrap_or (link) ; let link = link . strip_suffix ("{}") . unwrap_or (link) ; let link = link . strip_suffix ("[]") . unwrap_or (link) ; let link = if link != "!" { link . strip_suffix ('!') . unwrap_or (link) } else { link } ; let link = link . trim () ; strip_generics_from_path (link) . unwrap_or_else (| _ | link . into ()) }
}

macro_rules! may_be_doc_link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function may_be_doc_link in module {}", module_path!());
    };
}

mkfn!{
    may_be_doc_link_introspect!();
    #[doc = " Keep inline and reference links `[]`,"] #[doc = " but skip autolinks `<>` which we never consider to be intra-doc links."] pub fn may_be_doc_link (link_type : LinkType) -> bool { match link_type { LinkType :: Inline | LinkType :: Reference | LinkType :: ReferenceUnknown | LinkType :: Collapsed | LinkType :: CollapsedUnknown | LinkType :: Shortcut | LinkType :: ShortcutUnknown => true , LinkType :: Autolink | LinkType :: Email => false , } }
}

macro_rules! attrs_to_preprocessed_links_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function attrs_to_preprocessed_links in module {}", module_path!());
    };
}

mkfn!{
    attrs_to_preprocessed_links_introspect!();
    #[doc = " Simplified version of `preprocessed_markdown_links` from rustdoc."] #[doc = " Must return at least the same links as it, but may add some more links on top of that."] pub (crate) fn attrs_to_preprocessed_links < A : AttributeExt + Clone > (attrs : & [A]) -> Vec < Box < str > > { let (doc_fragments , _) = attrs_to_doc_fragments (attrs . iter () . map (| attr | (attr , None)) , true) ; let doc = prepare_to_doc_link_resolution (& doc_fragments) . into_values () . next () . unwrap () ; parse_links (& doc) }
}

macro_rules! parse_links_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_links in module {}", module_path!());
    };
}

mkfn!{
    parse_links_introspect!();
    #[doc = " Similar version of `markdown_links` from rustdoc."] #[doc = " This will collect destination links and display text if exists."] fn parse_links < 'md > (doc : & 'md str) -> Vec < Box < str > > { let mut broken_link_callback = | link : BrokenLink < 'md > | Some ((link . reference , "" . into ())) ; let mut event_iter = Parser :: new_with_broken_link_callback (doc , main_body_opts () , Some (& mut broken_link_callback) ,) ; let mut links = Vec :: new () ; let mut refids = UnordSet :: default () ; while let Some (event) = event_iter . next () { match event { Event :: Start (Tag :: Link { link_type , dest_url , title : _ , id }) if may_be_doc_link (link_type) => { if matches ! (link_type , LinkType :: Inline | LinkType :: ReferenceUnknown | LinkType :: Reference | LinkType :: Shortcut | LinkType :: ShortcutUnknown) { if let Some (display_text) = collect_link_data (& mut event_iter) { links . push (display_text) ; } } if matches ! (link_type , LinkType :: Reference | LinkType :: Shortcut | LinkType :: Collapsed) { refids . insert (id) ; } links . push (preprocess_link (& dest_url)) ; } _ => { } } } for (label , refdef) in event_iter . reference_definitions () . iter () . sorted_by_key (| x | x . 0) { if ! refids . contains (label) { links . push (preprocess_link (& refdef . dest)) ; } } links }
}

macro_rules! collect_link_data_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_link_data in module {}", module_path!());
    };
}

mkfn!{
    collect_link_data_introspect!();
    #[doc = " Collects additional data of link."] fn collect_link_data < 'input , F : BrokenLinkCallback < 'input > > (event_iter : & mut Parser < 'input , F > ,) -> Option < Box < str > > { let mut display_text : Option < String > = None ; let mut append_text = | text : CowStr < '_ > | { if let Some (display_text) = & mut display_text { display_text . push_str (& text) ; } else { display_text = Some (text . to_string ()) ; } } ; while let Some (event) = event_iter . next () { match event { Event :: Text (text) => { append_text (text) ; } Event :: Code (code) => { append_text (code) ; } Event :: End (_) => { break ; } _ => { } } } display_text . map (String :: into_boxed_str) }
}

macro_rules! span_of_fragments_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function span_of_fragments in module {}", module_path!());
    };
}

mkfn!{
    span_of_fragments_introspect!();
    #[doc = " Returns a span encompassing all the document fragments."] pub fn span_of_fragments (fragments : & [DocFragment]) -> Option < Span > { let (first_fragment , last_fragment) = match fragments { [] => return None , [first , .. , last] => (first , last) , [first] => (first , first) , } ; if first_fragment . span == DUMMY_SP { return None ; } Some (first_fragment . span . to (last_fragment . span)) }
}

macro_rules! source_span_for_markdown_range_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function source_span_for_markdown_range in module {}", module_path!());
    };
}

mkfn!{
    source_span_for_markdown_range_introspect!();
    #[doc = " Attempts to match a range of bytes from parsed markdown to a `Span` in the source code."] #[doc = ""] #[doc = " This method does not always work, because markdown bytes don't necessarily match source bytes,"] #[doc = " like if escapes are used in the string. In this case, it returns `None`."] #[doc = ""] #[doc = " `markdown` is typically the entire documentation for an item,"] #[doc = " after combining fragments."] #[doc = ""] #[doc = " This method will return `Some` only if one of the following is true:"] #[doc = ""] #[doc = " - The doc is made entirely from sugared doc comments, which cannot contain escapes"] #[doc = " - The doc is entirely from a single doc fragment with a string literal exactly equal to"] #[doc = "   `markdown`."] #[doc = " - The doc comes from `include_str!`"] #[doc = " - The doc includes exactly one substring matching `markdown[md_range]` which is contained in a"] #[doc = "   single doc fragment."] #[doc = ""] #[doc = " This function is defined in the compiler so it can be used by both `rustdoc` and `clippy`."] #[doc = ""] #[doc = " It returns a tuple containing a span encompassing all the document fragments and a boolean that"] #[doc = " is `true` if any of the *matched* fragments are from a macro expansion."] pub fn source_span_for_markdown_range (tcx : TyCtxt < '_ > , markdown : & str , md_range : & Range < usize > , fragments : & [DocFragment] ,) -> Option < (Span , bool) > { let map = tcx . sess . source_map () ; source_span_for_markdown_range_inner (map , markdown , md_range , fragments) }
}

macro_rules! source_span_for_markdown_range_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function source_span_for_markdown_range_inner in module {}", module_path!());
    };
}

mkfn!{
    source_span_for_markdown_range_inner_introspect!();
    pub fn source_span_for_markdown_range_inner (map : & SourceMap , markdown : & str , md_range : & Range < usize > , fragments : & [DocFragment] ,) -> Option < (Span , bool) > { use rustc_span :: BytePos ; if let & [fragment] = & fragments && fragment . kind == DocFragmentKind :: RawDoc && let Ok (snippet) = map . span_to_snippet (fragment . span) && snippet . trim_end () == markdown . trim_end () && let Ok (md_range_lo) = u32 :: try_from (md_range . start) && let Ok (md_range_hi) = u32 :: try_from (md_range . end) { return Some ((Span :: new (fragment . span . lo () + rustc_span :: BytePos (md_range_lo) , fragment . span . lo () + rustc_span :: BytePos (md_range_hi) , fragment . span . ctxt () , fragment . span . parent () ,) , fragment . from_expansion ,)) ; } let is_all_sugared_doc = fragments . iter () . all (| frag | frag . kind == DocFragmentKind :: SugaredDoc) ; if ! is_all_sugared_doc { let mut match_data = None ; let pat = & markdown [md_range . clone ()] ; if pat . is_empty () { return None ; } for (i , fragment) in fragments . iter () . enumerate () { if let Ok (snippet) = map . span_to_snippet (fragment . span) && let Some (match_start) = snippet . find (pat) { if match_data . is_none () && ! snippet . as_bytes () [match_start + 1 ..] . windows (pat . len ()) . any (| s | s == pat . as_bytes ()) { match_data = Some ((i , match_start)) ; } else { return None ; } } } if let Some ((i , match_start)) = match_data { let fragment = & fragments [i] ; let sp = fragment . span ; let lo = sp . lo () + BytePos (match_start as u32) ; return Some ((sp . with_lo (lo) . with_hi (lo + BytePos ((md_range . end - md_range . start) as u32)) , fragment . from_expansion ,)) ; } return None ; } let snippet = map . span_to_snippet (span_of_fragments (fragments) ?) . ok () ? ; let starting_line = markdown [.. md_range . start] . matches ('\n') . count () ; let ending_line = starting_line + markdown [md_range . start .. md_range . end] . matches ('\n') . count () ; let mut src_lines = snippet . split_terminator ('\n') ; let md_lines = markdown . split_terminator ('\n') ; let mut start_bytes = 0 ; let mut end_bytes = 0 ; 'outer : for (line_no , md_line) in md_lines . enumerate () { loop { let source_line = src_lines . next () ? ; match source_line . find (md_line) { Some (offset) => { if line_no == starting_line { start_bytes += offset ; if starting_line == ending_line { break 'outer ; } } else if line_no == ending_line { end_bytes += offset ; break 'outer ; } else if line_no < starting_line { start_bytes += source_line . len () - md_line . len () ; } else { end_bytes += source_line . len () - md_line . len () ; } break ; } None => { if line_no <= starting_line { start_bytes += source_line . len () + 1 ; } else { end_bytes += source_line . len () + 1 ; } } } } } let span = span_of_fragments (fragments) ? ; let src_span = span . from_inner (InnerSpan :: new (md_range . start + start_bytes , md_range . end + start_bytes + end_bytes ,)) ; Some ((src_span , fragments . iter () . any (| frag | frag . span . overlaps (src_span) && frag . from_expansion) ,)) }
}