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
mkuse!{use std :: iter ;}
mkuse!{use rustc_abi :: { CanonAbi , ExternAbi } ;}
mkuse!{use rustc_ast :: util :: parser :: ExprPrecedence ;}
mkuse!{use rustc_errors :: { Applicability , Diag , ErrorGuaranteed , StashKey } ;}
mkuse!{use rustc_hir :: def :: { self , CtorKind , Namespace , Res } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: { self as hir , HirId , LangItem } ;}
mkuse!{use rustc_hir_analysis :: autoderef :: Autoderef ;}
mkuse!{use rustc_infer :: infer :: BoundRegionConversionTime ;}
mkuse!{use rustc_infer :: traits :: { Obligation , ObligationCause , ObligationCauseCode } ;}
mkuse!{use rustc_middle :: ty :: adjustment :: { Adjust , Adjustment , AllowTwoPhase , AutoBorrow , AutoBorrowMutability , } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgsRef , Ty , TyCtxt , TypeVisitableExt } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use rustc_span :: { Span , sym } ;}
mkuse!{use rustc_target :: spec :: { AbiMap , AbiMapping } ;}
mkuse!{use rustc_trait_selection :: error_reporting :: traits :: DefIdOrName ;}
mkuse!{use rustc_trait_selection :: infer :: InferCtxtExt as _ ;}
mkuse!{use rustc_trait_selection :: traits :: query :: evaluate_obligation :: InferCtxtExt as _ ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: method :: MethodCallee ;}
mkuse!{use super :: method :: probe :: ProbeScope ;}
mkuse!{use super :: { Expectation , FnCtxt , TupleArgumentsFlag } ;}
mkuse!{use crate :: { errors , fluent_generated } ;}

macro_rules! check_legal_trait_for_method_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_legal_trait_for_method_call in module {}", module_path!());
    };
}

mkfn!{
    check_legal_trait_for_method_call_introspect!();
    #[doc = " Checks that it is legal to call methods of the trait corresponding"] #[doc = " to `trait_id` (this only cares about the trait, not the specific"] #[doc = " method that is called)."] pub (crate) fn check_legal_trait_for_method_call (tcx : TyCtxt < '_ > , span : Span , receiver : Option < Span > , expr_span : Span , trait_id : DefId , _body_id : DefId ,) -> Result < () , ErrorGuaranteed > { if tcx . is_lang_item (trait_id , LangItem :: Drop) { let sugg = if let Some (receiver) = receiver . filter (| s | ! s . is_empty ()) { errors :: ExplicitDestructorCallSugg :: Snippet { lo : expr_span . shrink_to_lo () , hi : receiver . shrink_to_hi () . to (expr_span . shrink_to_hi ()) , } } else { errors :: ExplicitDestructorCallSugg :: Empty (span) } ; return Err (tcx . dcx () . emit_err (errors :: ExplicitDestructorCall { span , sugg })) ; } tcx . ensure_ok () . coherent_trait (trait_id) }
}
mkitem!{mkenum!{#[derive (Debug)] enum CallStep < 'tcx > { Builtin (Ty < 'tcx >) , DeferredClosure (LocalDefId , ty :: FnSig < 'tcx >) , #[doc = " Call overloading when callee implements one of the Fn* traits."] Overloaded (MethodCallee < 'tcx >) , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { pub (crate) fn check_expr_call (& self , call_expr : & 'tcx hir :: Expr < 'tcx > , callee_expr : & 'tcx hir :: Expr < 'tcx > , arg_exprs : & 'tcx [hir :: Expr < 'tcx >] , expected : Expectation < 'tcx > ,) -> Ty < 'tcx > { let original_callee_ty = match & callee_expr . kind { hir :: ExprKind :: Path (hir :: QPath :: Resolved (..) | hir :: QPath :: TypeRelative (..)) => self . check_expr_with_expectation_and_args (callee_expr , Expectation :: NoExpectation , Some ((call_expr , arg_exprs)) ,) , _ => self . check_expr (callee_expr) , } ; let expr_ty = self . structurally_resolve_type (call_expr . span , original_callee_ty) ; let mut autoderef = self . autoderef (callee_expr . span , expr_ty) ; let mut result = None ; while result . is_none () && autoderef . next () . is_some () { result = self . try_overloaded_call_step (call_expr , callee_expr , arg_exprs , & autoderef) ; } match autoderef . final_ty () . kind () { ty :: FnDef (def_id , _) => { let abi = self . tcx . fn_sig (def_id) . skip_binder () . skip_binder () . abi ; self . check_call_abi (abi , call_expr . span) ; } ty :: FnPtr (_ , header) => { self . check_call_abi (header . abi , call_expr . span) ; } _ => { } } self . register_predicates (autoderef . into_obligations ()) ; let output = match result { None => { for arg in arg_exprs { self . check_expr (arg) ; } if let hir :: ExprKind :: Path (hir :: QPath :: Resolved (_ , path)) = & callee_expr . kind && let [segment] = path . segments { self . dcx () . try_steal_modify_and_emit_err (segment . ident . span , StashKey :: CallIntoMethod , | err | { self . suggest_call_as_method (err , segment , arg_exprs , call_expr , expected ,) ; } ,) ; } let guar = self . report_invalid_callee (call_expr , callee_expr , expr_ty , arg_exprs) ; Ty :: new_error (self . tcx , guar) } Some (CallStep :: Builtin (callee_ty)) => { self . confirm_builtin_call (call_expr , callee_expr , callee_ty , arg_exprs , expected) } Some (CallStep :: DeferredClosure (def_id , fn_sig)) => { self . confirm_deferred_closure_call (call_expr , arg_exprs , expected , def_id , fn_sig) } Some (CallStep :: Overloaded (method_callee)) => { self . confirm_overloaded_call (call_expr , arg_exprs , expected , method_callee) } } ; self . register_wf_obligation (output . into () , call_expr . span , ObligationCauseCode :: WellFormed (None) ,) ; output } #[doc = " Can a function with this ABI be called with a rust call expression?"] #[doc = ""] #[doc = " Some ABIs cannot be called from rust, either because rust does not know how to generate"] #[doc = " code for the call, or because a call does not semantically make sense."] pub (crate) fn check_call_abi (& self , abi : ExternAbi , span : Span) { let canon_abi = match AbiMap :: from_target (& self . sess () . target) . canonize_abi (abi , false) { AbiMapping :: Direct (canon_abi) | AbiMapping :: Deprecated (canon_abi) => canon_abi , AbiMapping :: Invalid => { let guar = self . dcx () . span_delayed_bug (span , format ! ("invalid abi for platform should have reported an error: {abi}") ,) ; self . set_tainted_by_errors (guar) ; return ; } } ; let valid = match canon_abi { CanonAbi :: Custom => false , CanonAbi :: GpuKernel => false , CanonAbi :: Interrupt (_) => false , CanonAbi :: C | CanonAbi :: Rust | CanonAbi :: RustCold | CanonAbi :: Arm (_) | CanonAbi :: X86 (_) => true , } ; if ! valid { let err = crate :: errors :: AbiCannotBeCalled { span , abi } ; self . tcx . dcx () . emit_err (err) ; } } #[instrument (level = "debug" , skip (self , call_expr , callee_expr , arg_exprs , autoderef) , ret)] fn try_overloaded_call_step (& self , call_expr : & 'tcx hir :: Expr < 'tcx > , callee_expr : & 'tcx hir :: Expr < 'tcx > , arg_exprs : & 'tcx [hir :: Expr < 'tcx >] , autoderef : & Autoderef < 'a , 'tcx > ,) -> Option < CallStep < 'tcx > > { let adjusted_ty = self . structurally_resolve_type (autoderef . span () , autoderef . final_ty ()) ; match * adjusted_ty . kind () { ty :: FnDef (..) | ty :: FnPtr (..) => { let adjustments = self . adjust_steps (autoderef) ; self . apply_adjustments (callee_expr , adjustments) ; return Some (CallStep :: Builtin (adjusted_ty)) ; } ty :: Closure (def_id , args) if self . closure_kind (adjusted_ty) . is_none () => { let def_id = def_id . expect_local () ; let closure_sig = args . as_closure () . sig () ; let closure_sig = self . instantiate_binder_with_fresh_vars (call_expr . span , BoundRegionConversionTime :: FnCall , closure_sig ,) ; let adjustments = self . adjust_steps (autoderef) ; self . record_deferred_call_resolution (def_id , DeferredCallResolution { call_expr , callee_expr , closure_ty : adjusted_ty , adjustments , fn_sig : closure_sig , } ,) ; return Some (CallStep :: DeferredClosure (def_id , closure_sig)) ; } ty :: CoroutineClosure (def_id , args) if self . closure_kind (adjusted_ty) . is_none () => { let def_id = def_id . expect_local () ; let closure_args = args . as_coroutine_closure () ; let coroutine_closure_sig = self . instantiate_binder_with_fresh_vars (call_expr . span , BoundRegionConversionTime :: FnCall , closure_args . coroutine_closure_sig () ,) ; let tupled_upvars_ty = self . next_ty_var (callee_expr . span) ; let kind_ty = self . next_ty_var (callee_expr . span) ; let call_sig = self . tcx . mk_fn_sig ([coroutine_closure_sig . tupled_inputs_ty] , coroutine_closure_sig . to_coroutine (self . tcx , closure_args . parent_args () , kind_ty , self . tcx . coroutine_for_closure (def_id) , tupled_upvars_ty ,) , coroutine_closure_sig . c_variadic , coroutine_closure_sig . safety , coroutine_closure_sig . abi ,) ; let adjustments = self . adjust_steps (autoderef) ; self . record_deferred_call_resolution (def_id , DeferredCallResolution { call_expr , callee_expr , closure_ty : adjusted_ty , adjustments , fn_sig : call_sig , } ,) ; return Some (CallStep :: DeferredClosure (def_id , call_sig)) ; } ty :: Ref (..) if autoderef . step_count () == 0 => { return None ; } ty :: Error (_) => { return None ; } _ => { } } self . try_overloaded_call_traits (call_expr , adjusted_ty , Some (arg_exprs)) . or_else (| | self . try_overloaded_call_traits (call_expr , adjusted_ty , None)) . map (| (autoref , method) | { let mut adjustments = self . adjust_steps (autoderef) ; adjustments . extend (autoref) ; self . apply_adjustments (callee_expr , adjustments) ; CallStep :: Overloaded (method) }) } fn try_overloaded_call_traits (& self , call_expr : & hir :: Expr < '_ > , adjusted_ty : Ty < 'tcx > , opt_arg_exprs : Option < & 'tcx [hir :: Expr < 'tcx >] > ,) -> Option < (Option < Adjustment < 'tcx > > , MethodCallee < 'tcx >) > { let call_trait_choices = if self . shallow_resolve (adjusted_ty) . is_coroutine_closure () { [(self . tcx . lang_items () . async_fn_trait () , sym :: async_call , true) , (self . tcx . lang_items () . async_fn_mut_trait () , sym :: async_call_mut , true) , (self . tcx . lang_items () . async_fn_once_trait () , sym :: async_call_once , false) , (self . tcx . lang_items () . fn_trait () , sym :: call , true) , (self . tcx . lang_items () . fn_mut_trait () , sym :: call_mut , true) , (self . tcx . lang_items () . fn_once_trait () , sym :: call_once , false) ,] } else { [(self . tcx . lang_items () . fn_trait () , sym :: call , true) , (self . tcx . lang_items () . fn_mut_trait () , sym :: call_mut , true) , (self . tcx . lang_items () . fn_once_trait () , sym :: call_once , false) , (self . tcx . lang_items () . async_fn_trait () , sym :: async_call , true) , (self . tcx . lang_items () . async_fn_mut_trait () , sym :: async_call_mut , true) , (self . tcx . lang_items () . async_fn_once_trait () , sym :: async_call_once , false) ,] } ; for (opt_trait_def_id , method_name , borrow) in call_trait_choices { let Some (trait_def_id) = opt_trait_def_id else { continue } ; let opt_input_type = opt_arg_exprs . map (| arg_exprs | { Ty :: new_tup_from_iter (self . tcx , arg_exprs . iter () . map (| e | self . next_ty_var (e . span))) }) ; if let Some (ok) = self . lookup_method_for_operator (self . misc (call_expr . span) , method_name , trait_def_id , adjusted_ty , opt_input_type ,) { let method = self . register_infer_ok_obligations (ok) ; let mut autoref = None ; if borrow { let ty :: Ref (_ , _ , mutbl) = method . sig . inputs () [0] . kind () else { bug ! ("Expected `FnMut`/`Fn` to take receiver by-ref/by-mut") } ; let mutbl = AutoBorrowMutability :: new (* mutbl , AllowTwoPhase :: No) ; autoref = Some (Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (mutbl)) , target : method . sig . inputs () [0] , }) ; } return Some ((autoref , method)) ; } } None } #[doc = " Give appropriate suggestion when encountering `||{/* not callable */}()`, where the"] #[doc = " likely intention is to call the closure, suggest `(||{})()`. (#55851)"] fn identify_bad_closure_def_and_call (& self , err : & mut Diag < '_ > , hir_id : hir :: HirId , callee_node : & hir :: ExprKind < '_ > , callee_span : Span ,) { let hir :: ExprKind :: Block (..) = callee_node else { return ; } ; let fn_decl_span = if let hir :: Node :: Expr (& hir :: Expr { kind : hir :: ExprKind :: Closure (& hir :: Closure { fn_decl_span , .. }) , .. }) = self . tcx . parent_hir_node (hir_id) { fn_decl_span } else if let Some ((_ , hir :: Node :: Expr (& hir :: Expr { hir_id : parent_hir_id , kind : hir :: ExprKind :: Closure (& hir :: Closure { kind : hir :: ClosureKind :: Coroutine (hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: Async , hir :: CoroutineSource :: Closure ,)) , .. }) , .. }) ,)) = self . tcx . hir_parent_iter (hir_id) . nth (3) { if let hir :: Node :: Expr (& hir :: Expr { kind : hir :: ExprKind :: Closure (& hir :: Closure { fn_decl_span , .. }) , .. }) = self . tcx . parent_hir_node (parent_hir_id) { fn_decl_span } else { return ; } } else { return ; } ; let start = fn_decl_span . shrink_to_lo () ; let end = callee_span . shrink_to_hi () ; err . multipart_suggestion ("if you meant to create this closure and immediately call it, surround the \
                closure with parentheses" , vec ! [(start , "(" . to_string ()) , (end , ")" . to_string ())] , Applicability :: MaybeIncorrect ,) ; } #[doc = " Give appropriate suggestion when encountering `[(\"a\", 0) (\"b\", 1)]`, where the"] #[doc = " likely intention is to create an array containing tuples."] fn maybe_suggest_bad_array_definition (& self , err : & mut Diag < '_ > , call_expr : & 'tcx hir :: Expr < 'tcx > , callee_expr : & 'tcx hir :: Expr < 'tcx > ,) -> bool { let parent_node = self . tcx . parent_hir_node (call_expr . hir_id) ; if let (hir :: Node :: Expr (hir :: Expr { kind : hir :: ExprKind :: Array (_) , .. }) , hir :: ExprKind :: Tup (exp) , hir :: ExprKind :: Call (_ , args) ,) = (parent_node , & callee_expr . kind , & call_expr . kind) && args . len () == exp . len () { let start = callee_expr . span . shrink_to_hi () ; err . span_suggestion (start , "consider separating array elements with a comma" , "," , Applicability :: MaybeIncorrect ,) ; return true ; } false } fn confirm_builtin_call (& self , call_expr : & 'tcx hir :: Expr < 'tcx > , callee_expr : & 'tcx hir :: Expr < 'tcx > , callee_ty : Ty < 'tcx > , arg_exprs : & 'tcx [hir :: Expr < 'tcx >] , expected : Expectation < 'tcx > ,) -> Ty < 'tcx > { let (fn_sig , def_id) = match * callee_ty . kind () { ty :: FnDef (def_id , args) => { self . enforce_context_effects (Some (call_expr . hir_id) , call_expr . span , def_id , args) ; let fn_sig = self . tcx . fn_sig (def_id) . instantiate (self . tcx , args) ; #[allow (rustc :: untranslatable_diagnostic)] #[allow (rustc :: diagnostic_outside_of_impl)] if self . has_rustc_attrs && self . tcx . has_attr (def_id , sym :: rustc_evaluate_where_clauses) { let predicates = self . tcx . predicates_of (def_id) ; let predicates = predicates . instantiate (self . tcx , args) ; for (predicate , predicate_span) in predicates { let obligation = Obligation :: new (self . tcx , ObligationCause :: dummy_with_span (callee_expr . span) , self . param_env , predicate ,) ; let result = self . evaluate_obligation (& obligation) ; self . dcx () . struct_span_err (callee_expr . span , format ! ("evaluate({predicate:?}) = {result:?}") ,) . with_span_label (predicate_span , "predicate") . emit () ; } } (fn_sig , Some (def_id)) } ty :: FnPtr (sig_tys , hdr) => (sig_tys . with (hdr) , None) , _ => unreachable ! () , } ; let fn_sig = self . instantiate_binder_with_fresh_vars (call_expr . span , BoundRegionConversionTime :: FnCall , fn_sig ,) ; let fn_sig = self . normalize (call_expr . span , fn_sig) ; self . check_argument_types (call_expr . span , call_expr , fn_sig . inputs () , fn_sig . output () , expected , arg_exprs , fn_sig . c_variadic , TupleArgumentsFlag :: DontTupleArguments , def_id ,) ; if fn_sig . abi == rustc_abi :: ExternAbi :: RustCall { let sp = arg_exprs . last () . map_or (call_expr . span , | expr | expr . span) ; if let Some (ty) = fn_sig . inputs () . last () . copied () { self . register_bound (ty , self . tcx . require_lang_item (hir :: LangItem :: Tuple , sp) , self . cause (sp , ObligationCauseCode :: RustCall) ,) ; self . require_type_is_sized (ty , sp , ObligationCauseCode :: RustCall) ; } else { self . dcx () . emit_err (errors :: RustCallIncorrectArgs { span : sp }) ; } } fn_sig . output () } #[doc = " Attempts to reinterpret `method(rcvr, args...)` as `rcvr.method(args...)`"] #[doc = " and suggesting the fix if the method probe is successful."] fn suggest_call_as_method (& self , diag : & mut Diag < '_ > , segment : & 'tcx hir :: PathSegment < 'tcx > , arg_exprs : & 'tcx [hir :: Expr < 'tcx >] , call_expr : & 'tcx hir :: Expr < 'tcx > , expected : Expectation < 'tcx > ,) { if let [callee_expr , rest @ ..] = arg_exprs { let Some (callee_ty) = self . typeck_results . borrow () . expr_ty_adjusted_opt (callee_expr) else { return ; } ; let Ok (pick) = self . lookup_probe_for_diagnostic (segment . ident , callee_ty , call_expr , ProbeScope :: AllTraits , expected . only_has_type (self) ,) else { return ; } ; let pick = self . confirm_method_for_diagnostic (call_expr . span , callee_expr , call_expr , callee_ty , & pick , segment ,) ; if pick . illegal_sized_bound . is_some () { return ; } let Some (callee_expr_span) = callee_expr . span . find_ancestor_inside (call_expr . span) else { return ; } ; let up_to_rcvr_span = segment . ident . span . until (callee_expr_span) ; let rest_span = callee_expr_span . shrink_to_hi () . to (call_expr . span . shrink_to_hi ()) ; let rest_snippet = if let Some (first) = rest . first () { self . tcx . sess . source_map () . span_to_snippet (first . span . to (call_expr . span . shrink_to_hi ())) } else { Ok (")" . to_string ()) } ; if let Ok (rest_snippet) = rest_snippet { let sugg = if self . precedence (callee_expr) >= ExprPrecedence :: Unambiguous { vec ! [(up_to_rcvr_span , "" . to_string ()) , (rest_span , format ! (".{}({rest_snippet}" , segment . ident)) ,] } else { vec ! [(up_to_rcvr_span , "(" . to_string ()) , (rest_span , format ! (").{}({rest_snippet}" , segment . ident)) ,] } ; let self_ty = self . resolve_vars_if_possible (pick . callee . sig . inputs () [0]) ; diag . multipart_suggestion (format ! ("use the `.` operator to call the method `{}{}` on `{self_ty}`" , self . tcx . associated_item (pick . callee . def_id) . trait_container (self . tcx) . map_or_else (|| String :: new () , | trait_def_id | self . tcx . def_path_str (trait_def_id) + "::") , segment . ident) , sugg , Applicability :: MaybeIncorrect ,) ; } } } fn report_invalid_callee (& self , call_expr : & 'tcx hir :: Expr < 'tcx > , callee_expr : & 'tcx hir :: Expr < 'tcx > , callee_ty : Ty < 'tcx > , arg_exprs : & 'tcx [hir :: Expr < 'tcx >] ,) -> ErrorGuaranteed { if let Some ((_ , _ , args)) = self . extract_callable_info (callee_ty) && let Err (err) = args . error_reported () { return err ; } let mut unit_variant = None ; if let hir :: ExprKind :: Path (qpath) = & callee_expr . kind && let Res :: Def (def :: DefKind :: Ctor (kind , CtorKind :: Const) , _) = self . typeck_results . borrow () . qpath_res (qpath , callee_expr . hir_id) && arg_exprs . is_empty () && call_expr . span . contains (callee_expr . span) { let descr = match kind { def :: CtorOf :: Struct => "struct" , def :: CtorOf :: Variant => "enum variant" , } ; let removal_span = callee_expr . span . shrink_to_hi () . to (call_expr . span . shrink_to_hi ()) ; unit_variant = Some ((removal_span , descr , rustc_hir_pretty :: qpath_to_string (& self . tcx , qpath))) ; } let callee_ty = self . resolve_vars_if_possible (callee_ty) ; let mut path = None ; let mut err = self . dcx () . create_err (errors :: InvalidCallee { span : callee_expr . span , ty : callee_ty , found : match & unit_variant { Some ((_ , kind , path)) => format ! ("{kind} `{path}`") , None => format ! ("`{}`" , self . tcx . short_string (callee_ty , & mut path)) , } , }) ; * err . long_ty_path () = path ; if callee_ty . references_error () { err . downgrade_to_delayed_bug () ; } self . identify_bad_closure_def_and_call (& mut err , call_expr . hir_id , & callee_expr . kind , callee_expr . span ,) ; if let Some ((removal_span , kind , path)) = & unit_variant { err . span_suggestion_verbose (* removal_span , format ! ("`{path}` is a unit {kind}, and does not take parentheses to be constructed" ,) , "" , Applicability :: MachineApplicable ,) ; } if let hir :: ExprKind :: Path (hir :: QPath :: Resolved (None , path)) = callee_expr . kind && let Res :: Local (_) = path . res && let [segment] = & path . segments { for id in self . tcx . hir_free_items () { if let Some (node) = self . tcx . hir_get_if_local (id . owner_id . into ()) && let hir :: Node :: Item (item) = node && let hir :: ItemKind :: Fn { ident , .. } = item . kind && ident . name == segment . ident . name { err . span_label (self . tcx . def_span (id . owner_id) , "this function of the same name is available here, but it's shadowed by \
                         the local binding" ,) ; } } } let mut inner_callee_path = None ; let def = match callee_expr . kind { hir :: ExprKind :: Path (ref qpath) => { self . typeck_results . borrow () . qpath_res (qpath , callee_expr . hir_id) } hir :: ExprKind :: Call (inner_callee , _) => { if let hir :: ExprKind :: Path (ref inner_qpath) = inner_callee . kind { inner_callee_path = Some (inner_qpath) ; self . typeck_results . borrow () . qpath_res (inner_qpath , inner_callee . hir_id) } else { Res :: Err } } _ => Res :: Err , } ; if ! self . maybe_suggest_bad_array_definition (& mut err , call_expr , callee_expr) { let call_is_multiline = self . tcx . sess . source_map () . is_multiline (call_expr . span . with_lo (callee_expr . span . hi ())) && call_expr . span . eq_ctxt (callee_expr . span) ; if call_is_multiline { err . span_suggestion (callee_expr . span . shrink_to_hi () , "consider using a semicolon here to finish the statement" , ";" , Applicability :: MaybeIncorrect ,) ; } if let Some ((maybe_def , output_ty , _)) = self . extract_callable_info (callee_ty) && ! self . type_is_sized_modulo_regions (self . param_env , output_ty) { let descr = match maybe_def { DefIdOrName :: DefId (def_id) => self . tcx . def_descr (def_id) , DefIdOrName :: Name (name) => name , } ; err . span_label (callee_expr . span , format ! ("this {descr} returns an unsized value `{output_ty}`, so it cannot be called")) ; if let DefIdOrName :: DefId (def_id) = maybe_def && let Some (def_span) = self . tcx . hir_span_if_local (def_id) { err . span_label (def_span , "the callable type is defined here") ; } } else { err . span_label (call_expr . span , "call expression requires function") ; } } if let Some (span) = self . tcx . hir_res_span (def) { let callee_ty = callee_ty . to_string () ; let label = match (unit_variant , inner_callee_path) { (Some ((_ , kind , path)) , _) => { err . arg ("kind" , kind) ; err . arg ("path" , path) ; Some (fluent_generated :: hir_typeck_invalid_defined_kind) } (_ , Some (hir :: QPath :: Resolved (_ , path))) => { self . tcx . sess . source_map () . span_to_snippet (path . span) . ok () . map (| p | { err . arg ("func" , p) ; fluent_generated :: hir_typeck_invalid_fn_defined }) } _ => { match def { Res :: Local (hir_id) => { err . arg ("local_name" , self . tcx . hir_name (hir_id)) ; Some (fluent_generated :: hir_typeck_invalid_local) } Res :: Def (kind , def_id) if kind . ns () == Some (Namespace :: ValueNS) => { err . arg ("path" , self . tcx . def_path_str (def_id)) ; Some (fluent_generated :: hir_typeck_invalid_defined) } _ => { err . arg ("path" , callee_ty) ; Some (fluent_generated :: hir_typeck_invalid_defined) } } } } ; if let Some (label) = label { err . span_label (span , label) ; } } err . emit () } fn confirm_deferred_closure_call (& self , call_expr : & 'tcx hir :: Expr < 'tcx > , arg_exprs : & 'tcx [hir :: Expr < 'tcx >] , expected : Expectation < 'tcx > , closure_def_id : LocalDefId , fn_sig : ty :: FnSig < 'tcx > ,) -> Ty < 'tcx > { self . check_argument_types (call_expr . span , call_expr , fn_sig . inputs () , fn_sig . output () , expected , arg_exprs , fn_sig . c_variadic , TupleArgumentsFlag :: TupleArguments , Some (closure_def_id . to_def_id ()) ,) ; fn_sig . output () } #[tracing :: instrument (level = "debug" , skip (self , span))] pub (super) fn enforce_context_effects (& self , call_hir_id : Option < HirId > , span : Span , callee_did : DefId , callee_args : GenericArgsRef < 'tcx > ,) { if ! self . tcx . features () . const_trait_impl () { return ; } if self . has_rustc_attrs && self . tcx . has_attr (self . body_id , sym :: rustc_do_not_const_check) { return ; } let host = match self . tcx . hir_body_const_context (self . body_id) { Some (hir :: ConstContext :: Const { .. } | hir :: ConstContext :: Static (_)) => { ty :: BoundConstness :: Const } Some (hir :: ConstContext :: ConstFn) => ty :: BoundConstness :: Maybe , None => return , } ; if self . tcx . is_conditionally_const (callee_did) { let q = self . tcx . const_conditions (callee_did) ; for (idx , (cond , pred_span)) in q . instantiate (self . tcx , callee_args) . into_iter () . enumerate () { let cause = self . cause (span , if let Some (hir_id) = call_hir_id { ObligationCauseCode :: HostEffectInExpr (callee_did , pred_span , hir_id , idx) } else { ObligationCauseCode :: WhereClause (callee_did , pred_span) } ,) ; self . register_predicate (Obligation :: new (self . tcx , cause , self . param_env , cond . to_host_effect_clause (self . tcx , host) ,)) ; } } else { } } fn confirm_overloaded_call (& self , call_expr : & 'tcx hir :: Expr < 'tcx > , arg_exprs : & 'tcx [hir :: Expr < 'tcx >] , expected : Expectation < 'tcx > , method : MethodCallee < 'tcx > ,) -> Ty < 'tcx > { self . check_argument_types (call_expr . span , call_expr , & method . sig . inputs () [1 ..] , method . sig . output () , expected , arg_exprs , method . sig . c_variadic , TupleArgumentsFlag :: TupleArguments , Some (method . def_id) ,) ; self . write_method_call_and_enforce_effects (call_expr . hir_id , call_expr . span , method) ; method . sig . output () } }}}
mkitem!{mkstruct!{#[derive (Debug)] pub (crate) struct DeferredCallResolution < 'tcx > { call_expr : & 'tcx hir :: Expr < 'tcx > , callee_expr : & 'tcx hir :: Expr < 'tcx > , closure_ty : Ty < 'tcx > , adjustments : Vec < Adjustment < 'tcx > > , fn_sig : ty :: FnSig < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > DeferredCallResolution < 'tcx > { pub (crate) fn resolve (self , fcx : & FnCtxt < 'a , 'tcx >) { debug ! ("DeferredCallResolution::resolve() {:?}" , self) ; assert ! (fcx . closure_kind (self . closure_ty) . is_some ()) ; match fcx . try_overloaded_call_traits (self . call_expr , self . closure_ty , None) { Some ((autoref , method_callee)) => { let method_sig = method_callee . sig ; debug ! ("attempt_resolution: method_callee={:?}" , method_callee) ; for (method_arg_ty , self_arg_ty) in iter :: zip (method_sig . inputs () . iter () . skip (1) , self . fn_sig . inputs ()) { fcx . demand_eqtype (self . call_expr . span , * self_arg_ty , * method_arg_ty) ; } fcx . demand_eqtype (self . call_expr . span , method_sig . output () , self . fn_sig . output ()) ; let mut adjustments = self . adjustments ; adjustments . extend (autoref) ; fcx . apply_adjustments (self . callee_expr , adjustments) ; fcx . write_method_call_and_enforce_effects (self . call_expr . hir_id , self . call_expr . span , method_callee ,) ; } None => { span_bug ! (self . call_expr . span , "Expected to find a suitable `Fn`/`FnMut`/`FnOnce` implementation for `{}`" , self . closure_ty) } } } }}}