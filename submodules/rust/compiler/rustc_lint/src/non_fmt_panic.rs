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
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_errors :: Applicability ;}
mkuse!{use rustc_hir :: { self as hir , LangItem } ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_middle :: { bug , ty } ;}
mkuse!{use rustc_parse_format :: { ParseMode , Parser , Piece } ;}
mkuse!{use rustc_session :: lint :: FutureIncompatibilityReason ;}
mkuse!{use rustc_session :: { declare_lint , declare_lint_pass } ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: { InnerSpan , Span , Symbol , hygiene , sym } ;}
mkuse!{use rustc_trait_selection :: infer :: InferCtxtExt ;}
mkuse!{use crate :: lints :: { NonFmtPanicBraces , NonFmtPanicUnused } ;}
mkuse!{use crate :: { LateContext , LateLintPass , LintContext , fluent_generated as fluent } ;}
mkitem!{declare_lint ! { # [doc = " The `non_fmt_panics` lint detects `panic!(..)` invocations where the first"] # [doc = " argument is not a formatting string."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,no_run,edition2018"] # [doc = " panic!(\"{}\");"] # [doc = " panic!(123);"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " In Rust 2018 and earlier, `panic!(x)` directly uses `x` as the message."] # [doc = " That means that `panic!(\"{}\")` panics with the message `\"{}\"` instead"] # [doc = " of using it as a formatting string, and `panic!(123)` will panic with"] # [doc = " an `i32` as message."] # [doc = ""] # [doc = " Rust 2021 always interprets the first argument as format string."] NON_FMT_PANICS , Warn , "detect single-argument panic!() invocations in which the argument is not a format string" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionSemanticsChange (Edition :: Edition2021) , explain_reason : false , } ; report_in_external_macro }}
mkitem!{declare_lint_pass ! (NonPanicFmt => [NON_FMT_PANICS]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for NonPanicFmt { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Call (f , [arg]) = & expr . kind && let & ty :: FnDef (def_id , _) = cx . typeck_results () . expr_ty (f) . kind () { let f_diagnostic_name = cx . tcx . get_diagnostic_name (def_id) ; if cx . tcx . is_lang_item (def_id , LangItem :: BeginPanic) || cx . tcx . is_lang_item (def_id , LangItem :: Panic) || f_diagnostic_name == Some (sym :: panic_str_2015) { if let Some (id) = f . span . ctxt () . outer_expn_data () . macro_def_id { if matches ! (cx . tcx . get_diagnostic_name (id) , Some (sym :: core_panic_2015_macro | sym :: std_panic_2015_macro)) { check_panic (cx , f , arg) ; } } } else if f_diagnostic_name == Some (sym :: unreachable_display) { if let Some (id) = f . span . ctxt () . outer_expn_data () . macro_def_id && cx . tcx . is_diagnostic_item (sym :: unreachable_2015_macro , id) { check_panic (cx , f , match & arg . kind { hir :: ExprKind :: AddrOf (ast :: BorrowKind :: Ref , _ , arg) => arg , _ => bug ! ("call to unreachable_display without borrow") , } ,) ; } } } } }}}

macro_rules! check_panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_panic in module {}", module_path!());
    };
}

mkfn!{
    check_panic_introspect!();
    fn check_panic < 'tcx > (cx : & LateContext < 'tcx > , f : & 'tcx hir :: Expr < 'tcx > , arg : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Lit (lit) = & arg . kind { if let ast :: LitKind :: Str (sym , _) = lit . node { check_panic_str (cx , f , arg , sym . as_str ()) ; return ; } } let (span , panic , symbol) = panic_call (cx , f) ; if span . in_external_macro (cx . sess () . source_map ()) { return ; } let mut arg_span = arg . span ; let mut arg_macro = None ; while ! span . contains (arg_span) { let ctxt = arg_span . ctxt () ; if ctxt . is_root () { break ; } let expn = ctxt . outer_expn_data () ; arg_macro = expn . macro_def_id ; arg_span = expn . call_site ; } # [allow (rustc :: diagnostic_outside_of_impl)] cx . span_lint (NON_FMT_PANICS , arg_span , | lint | { lint . primary_message (fluent :: lint_non_fmt_panic) ; lint . arg ("name" , symbol) ; lint . note (fluent :: lint_note) ; lint . note (fluent :: lint_more_info_note) ; if ! is_arg_inside_call (arg_span , span) { return ; } if arg_macro . is_some_and (| id | cx . tcx . is_diagnostic_item (sym :: format_macro , id)) { lint . note (fluent :: lint_supports_fmt_note) ; if let Some ((open , close , _)) = find_delimiters (cx , arg_span) { lint . multipart_suggestion (fluent :: lint_supports_fmt_suggestion , vec ! [(arg_span . until (open . shrink_to_hi ()) , "" . into ()) , (close . until (arg_span . shrink_to_hi ()) , "" . into ()) ,] , Applicability :: MachineApplicable ,) ; } } else { let ty = cx . typeck_results () . expr_ty (arg) ; let is_str = matches ! (ty . kind () , ty :: Ref (_ , r , _) if r . is_str () ,) || matches ! (ty . ty_adt_def () , Some (ty_def) if cx . tcx . is_lang_item (ty_def . did () , LangItem :: String) ,) ; let (infcx , param_env) = cx . tcx . infer_ctxt () . build_with_typing_env (cx . typing_env ()) ; let suggest_display = is_str || cx . tcx . get_diagnostic_item (sym :: Display) . is_some_and (| t | infcx . type_implements_trait (t , [ty] , param_env) . may_apply ()) ; let suggest_debug = ! suggest_display && cx . tcx . get_diagnostic_item (sym :: Debug) . is_some_and (| t | infcx . type_implements_trait (t , [ty] , param_env) . may_apply ()) ; let suggest_panic_any = ! is_str && panic == Some (sym :: std_panic_macro) ; let fmt_applicability = if suggest_panic_any { Applicability :: MaybeIncorrect } else { Applicability :: MachineApplicable } ; if suggest_display { lint . span_suggestion_verbose (arg_span . shrink_to_lo () , fluent :: lint_display_suggestion , "\"{}\", " , fmt_applicability ,) ; } else if suggest_debug { lint . arg ("ty" , ty) ; lint . span_suggestion_verbose (arg_span . shrink_to_lo () , fluent :: lint_debug_suggestion , "\"{:?}\", " , fmt_applicability ,) ; } if suggest_panic_any { if let Some ((open , close , del)) = find_delimiters (cx , span) { lint . arg ("already_suggested" , suggest_display || suggest_debug) ; lint . multipart_suggestion (fluent :: lint_panic_suggestion , if del == '(' { vec ! [(span . until (open) , "std::panic::panic_any" . into ())] } else { vec ! [(span . until (open . shrink_to_hi ()) , "std::panic::panic_any(" . into ()) , (close , ")" . into ()) ,] } , Applicability :: MachineApplicable ,) ; } } } }) ; }
}

macro_rules! check_panic_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_panic_str in module {}", module_path!());
    };
}

mkfn!{
    check_panic_str_introspect!();
    fn check_panic_str < 'tcx > (cx : & LateContext < 'tcx > , f : & 'tcx hir :: Expr < 'tcx > , arg : & 'tcx hir :: Expr < 'tcx > , fmt : & str ,) { if ! fmt . contains (& ['{' , '}']) { return ; } let (span , _ , _) = panic_call (cx , f) ; let sm = cx . sess () . source_map () ; if span . in_external_macro (sm) && arg . span . in_external_macro (sm) { return ; } let fmt_span = arg . span . source_callsite () ; let (snippet , style) = match sm . span_to_snippet (fmt_span) { Ok (snippet) => { let style = snippet . strip_prefix ('r') . and_then (| s | s . find ('"')) ; (Some (snippet) , style) } Err (_) => (None , None) , } ; let mut fmt_parser = Parser :: new (fmt , style , snippet . clone () , false , ParseMode :: Format) ; let n_arguments = (& mut fmt_parser) . filter (| a | matches ! (a , Piece :: NextArgument (_))) . count () ; if n_arguments > 0 && fmt_parser . errors . is_empty () { let arg_spans : Vec < _ > = match & fmt_parser . arg_places [..] { [] => vec ! [fmt_span] , v => v . iter () . map (| span | fmt_span . from_inner (InnerSpan :: new (span . start , span . end))) . collect () , } ; cx . emit_span_lint (NON_FMT_PANICS , arg_spans , NonFmtPanicUnused { count : n_arguments , suggestion : is_arg_inside_call (arg . span , span) . then_some (arg . span) , } ,) ; } else { let brace_spans : Option < Vec < _ > > = snippet . filter (| s | s . starts_with ('"') || s . starts_with ("r#")) . map (| s | { s . char_indices () . filter (| & (_ , c) | c == '{' || c == '}') . map (| (i , _) | fmt_span . from_inner (InnerSpan { start : i , end : i + 1 })) . collect () }) ; let count = brace_spans . as_ref () . map (| v | v . len ()) . unwrap_or (2) ; cx . emit_span_lint (NON_FMT_PANICS , brace_spans . unwrap_or_else (| | vec ! [span]) , NonFmtPanicBraces { count , suggestion : is_arg_inside_call (arg . span , span) . then_some (arg . span . shrink_to_lo ()) , } ,) ; } }
}

macro_rules! find_delimiters_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_delimiters in module {}", module_path!());
    };
}

mkfn!{
    find_delimiters_introspect!();
    # [doc = " Given the span of `some_macro!(args);`, gives the span of `(` and `)`,"] # [doc = " and the type of (opening) delimiter used."] fn find_delimiters (cx : & LateContext < '_ > , span : Span) -> Option < (Span , Span , char) > { let snippet = cx . sess () . source_map () . span_to_snippet (span) . ok () ? ; let (open , open_ch) = snippet . char_indices () . find (| & (_ , c) | "([{" . contains (c)) ? ; let close = snippet . rfind (| c | ")]}" . contains (c)) ? ; Some ((span . from_inner (InnerSpan { start : open , end : open + 1 }) , span . from_inner (InnerSpan { start : close , end : close + 1 }) , open_ch ,)) }
}

macro_rules! panic_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_call in module {}", module_path!());
    };
}

mkfn!{
    panic_call_introspect!();
    fn panic_call < 'tcx > (cx : & LateContext < 'tcx > , f : & 'tcx hir :: Expr < 'tcx > ,) -> (Span , Option < Symbol > , Symbol) { let mut expn = f . span . ctxt () . outer_expn_data () ; let mut panic_macro = None ; loop { let parent = expn . call_site . ctxt () . outer_expn_data () ; let Some (id) = parent . macro_def_id else { break } ; let Some (name) = cx . tcx . get_diagnostic_name (id) else { break } ; if ! matches ! (name , sym :: core_panic_macro | sym :: std_panic_macro | sym :: assert_macro | sym :: debug_assert_macro | sym :: unreachable_macro) { break ; } expn = parent ; panic_macro = Some (name) ; } let macro_symbol = if let hygiene :: ExpnKind :: Macro (_ , symbol) = expn . kind { symbol } else { sym :: panic } ; (expn . call_site , panic_macro , macro_symbol) }
}

macro_rules! is_arg_inside_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_arg_inside_call in module {}", module_path!());
    };
}

mkfn!{
    is_arg_inside_call_introspect!();
    fn is_arg_inside_call (arg : Span , call : Span) -> bool { call . contains (arg) && ! call . source_equal (arg) }
}