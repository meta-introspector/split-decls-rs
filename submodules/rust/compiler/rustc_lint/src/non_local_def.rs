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
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (# [$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (# [$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{# [macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{# [macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { compile_error ! (concat ! ("USE|" , module_path ! () , "|" , stringify ! ($ use_stmt))) ; } ; }}
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
mkuse!{use rustc_errors :: MultiSpan ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor , VisitorExt } ;}
mkuse!{use rustc_hir :: { Body , HirId , Item , ItemKind , Node , Path , TyKind } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: { declare_lint , impl_lint_pass } ;}
mkuse!{use rustc_span :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{use rustc_span :: { ExpnKind , Span , kw , sym } ;}
mkuse!{use crate :: lints :: { NonLocalDefinitionsCargoUpdateNote , NonLocalDefinitionsDiag } ;}
mkuse!{use crate :: { LateContext , LateLintPass , LintContext , fluent_generated as fluent } ;}
mkitem!{declare_lint ! { # [doc = " The `non_local_definitions` lint checks for `impl` blocks and `#[macro_export]`"] # [doc = " macro inside bodies (functions, enum discriminant, ...)."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![warn(non_local_definitions)]"] # [doc = " trait MyTrait {}"] # [doc = " struct MyStruct;"] # [doc = ""] # [doc = " fn foo() {"] # [doc = "     impl MyTrait for MyStruct {}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Creating non-local definitions go against expectation and can create discrepancies"] # [doc = " in tooling. It should be avoided. It may become deny-by-default in edition 2024"] # [doc = " and higher, see the tracking issue <https://github.com/rust-lang/rust/issues/120363>."] # [doc = ""] # [doc = " An `impl` definition is non-local if it is nested inside an item and neither"] # [doc = " the type nor the trait are at the same nesting level as the `impl` block."] # [doc = ""] # [doc = " All nested bodies (functions, enum discriminant, array length, consts) (expect for"] # [doc = " `const _: Ty = { ... }` in top-level module, which is still undecided) are checked."] pub NON_LOCAL_DEFINITIONS , Warn , "checks for non-local definitions" , report_in_external_macro }}
mkitem!{mkstruct!{# [derive (Default)] pub (crate) struct NonLocalDefinitions { body_depth : u32 , }}}
mkitem!{impl_lint_pass ! (NonLocalDefinitions => [NON_LOCAL_DEFINITIONS]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for NonLocalDefinitions { fn check_body (& mut self , _cx : & LateContext < 'tcx > , _body : & Body < 'tcx >) { self . body_depth += 1 ; } fn check_body_post (& mut self , _cx : & LateContext < 'tcx > , _body : & Body < 'tcx >) { self . body_depth -= 1 ; } fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if self . body_depth == 0 { return ; } let def_id = item . owner_id . def_id . into () ; let parent = cx . tcx . parent (def_id) ; let parent_def_kind = cx . tcx . def_kind (parent) ; let parent_opt_item_name = cx . tcx . opt_item_name (parent) ; if self . body_depth == 1 && parent_def_kind == DefKind :: Const && parent_opt_item_name == Some (kw :: Underscore) { return ; } let cargo_update = | | { let oexpn = item . span . ctxt () . outer_expn_data () ; if let Some (def_id) = oexpn . macro_def_id && let ExpnKind :: Macro (macro_kind , macro_name) = oexpn . kind && def_id . krate != LOCAL_CRATE && rustc_session :: utils :: was_invoked_from_cargo () { Some (NonLocalDefinitionsCargoUpdateNote { macro_kind : macro_kind . descr () , macro_name , crate_name : cx . tcx . crate_name (def_id . krate) , }) } else { None } } ; let is_at_toplevel_doctest = | | { self . body_depth == 2 && cx . tcx . env_var_os ("UNSTABLE_RUSTDOC_TEST_PATH" . as_ref ()) . is_some () } ; match item . kind { ItemKind :: Impl (impl_) => { let mut collector = PathCollector { paths : Vec :: new () } ; collector . visit_ty_unambig (& impl_ . self_ty) ; if let Some (of_trait) = impl_ . of_trait { collector . visit_trait_ref (& of_trait . trait_ref) ; } collector . paths . retain (| p | matches ! (p . res , Res :: Def (def_kind , _) if def_kind != DefKind :: TyParam) ,) ; let outermost_impl_parent = peel_parent_while (cx . tcx , parent , | tcx , did | { tcx . def_kind (did) == DefKind :: Mod || (tcx . def_kind (did) == DefKind :: Const && tcx . opt_item_name (did) == Some (kw :: Underscore)) }) ; if collector . paths . iter () . any (| path | path_has_local_parent (path , cx , parent , outermost_impl_parent)) { return ; } let span_for_const_anon_suggestion = if parent_def_kind == DefKind :: Const && parent_opt_item_name != Some (kw :: Underscore) && let Some (parent) = parent . as_local () && let Node :: Item (item) = cx . tcx . hir_node_by_def_id (parent) && let ItemKind :: Const (ident , _ , ty , _) = item . kind && let TyKind :: Tup (& []) = ty . kind { Some (ident . span) } else { None } ; let const_anon = matches ! (parent_def_kind , DefKind :: Const | DefKind :: Static { .. }) . then_some (span_for_const_anon_suggestion) ; let impl_span = item . span . shrink_to_lo () . to (impl_ . self_ty . span) ; let mut ms = MultiSpan :: from_span (impl_span) ; for path in & collector . paths { # [allow (rustc :: untranslatable_diagnostic)] ms . push_span_label (path_span_without_args (path) , format ! ("`{}` is not local" , path_name_to_string (path)) ,) ; } let doctest = is_at_toplevel_doctest () ; if ! doctest { ms . push_span_label (cx . tcx . def_span (parent) , fluent :: lint_non_local_definitions_impl_move_help ,) ; } let macro_to_change = if let ExpnKind :: Macro (kind , name) = item . span . ctxt () . outer_expn_data () . kind { Some ((name . to_string () , kind . descr ())) } else { None } ; cx . emit_span_lint (NON_LOCAL_DEFINITIONS , ms , NonLocalDefinitionsDiag :: Impl { depth : self . body_depth , body_kind_descr : cx . tcx . def_kind_descr (parent_def_kind , parent) , body_name : parent_opt_item_name . map (| s | s . to_ident_string ()) . unwrap_or_else (| | "<unnameable>" . to_string ()) , cargo_update : cargo_update () , const_anon , doctest , macro_to_change , } ,) } ItemKind :: Macro (_ , _macro , _kinds) if cx . tcx . has_attr (item . owner_id . def_id , sym :: macro_export) => { cx . emit_span_lint (NON_LOCAL_DEFINITIONS , item . span , NonLocalDefinitionsDiag :: MacroRules { depth : self . body_depth , body_kind_descr : cx . tcx . def_kind_descr (parent_def_kind , parent) , body_name : parent_opt_item_name . map (| s | s . to_ident_string ()) . unwrap_or_else (| | "<unnameable>" . to_string ()) , cargo_update : cargo_update () , doctest : is_at_toplevel_doctest () , } ,) } _ => { } } } }}}
mkitem!{mkstruct!{# [doc = " Simple hir::Path collector"] struct PathCollector < 'tcx > { paths : Vec < Path < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for PathCollector < 'tcx > { fn visit_path (& mut self , path : & Path < 'tcx > , _id : HirId) { self . paths . push (path . clone ()) ; intravisit :: walk_path (self , path) } }}}

macro_rules! path_has_local_parent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_has_local_parent in module {}", module_path!());
    };
}

mkfn!{
    path_has_local_parent_introspect!();
    # [doc = " Given a path, this checks if the if the parent resolution def id corresponds to"] # [doc = " the def id of the parent impl definition (the direct one and the outermost one)."] # [doc = ""] # [doc = " Given this path, we will look at the path (and ignore any generic args):"] # [doc = ""] # [doc = " ```text"] # [doc = "    std::convert::PartialEq<Foo<Bar>>"] # [doc = "    ^^^^^^^^^^^^^^^^^^^^^^^"] # [doc = " ```"] # [inline] fn path_has_local_parent (path : & Path < '_ > , cx : & LateContext < '_ > , impl_parent : DefId , outermost_impl_parent : Option < DefId > ,) -> bool { path . res . opt_def_id () . is_some_and (| did | did_has_local_parent (did , cx . tcx , impl_parent , outermost_impl_parent)) }
}

macro_rules! did_has_local_parent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function did_has_local_parent in module {}", module_path!());
    };
}

mkfn!{
    did_has_local_parent_introspect!();
    # [doc = " Given a def id this checks if the parent def id (modulo modules) correspond to"] # [doc = " the def id of the parent impl definition (the direct one and the outermost one)."] # [inline] fn did_has_local_parent (did : DefId , tcx : TyCtxt < '_ > , impl_parent : DefId , outermost_impl_parent : Option < DefId > ,) -> bool { if ! did . is_local () { return false ; } let Some (parent_did) = tcx . opt_parent (did) else { return false ; } ; peel_parent_while (tcx , parent_did , | tcx , did | { tcx . def_kind (did) == DefKind :: Mod || (tcx . def_kind (did) == DefKind :: Const && tcx . opt_item_name (did) == Some (kw :: Underscore)) }) . map (| parent_did | parent_did == impl_parent || Some (parent_did) == outermost_impl_parent) . unwrap_or (false) }
}

macro_rules! peel_parent_while_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function peel_parent_while in module {}", module_path!());
    };
}

mkfn!{
    peel_parent_while_introspect!();
    # [doc = " Given a `DefId` checks if it satisfies `f` if it does check with it's parent and continue"] # [doc = " until it doesn't satisfies `f` and return the last `DefId` checked."] # [doc = ""] # [doc = " In other word this method return the first `DefId` that doesn't satisfies `f`."] # [inline] fn peel_parent_while (tcx : TyCtxt < '_ > , mut did : DefId , mut f : impl FnMut (TyCtxt < '_ > , DefId) -> bool ,) -> Option < DefId > { while ! did . is_crate_root () && f (tcx , did) { did = tcx . opt_parent (did) . filter (| parent_did | parent_did . is_local ()) ? ; } Some (did) }
}

macro_rules! path_span_without_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_span_without_args in module {}", module_path!());
    };
}

mkfn!{
    path_span_without_args_introspect!();
    # [doc = " Return for a given `Path` the span until the last args"] fn path_span_without_args (path : & Path < '_ >) -> Span { if let Some (args) = & path . segments . last () . unwrap () . args { path . span . until (args . span_ext) } else { path . span } }
}

macro_rules! path_name_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_name_to_string in module {}", module_path!());
    };
}

mkfn!{
    path_name_to_string_introspect!();
    # [doc = " Return a \"error message-able\" ident for the last segment of the `Path`"] fn path_name_to_string (path : & Path < '_ >) -> String { path . segments . last () . unwrap () . ident . to_string () }
}