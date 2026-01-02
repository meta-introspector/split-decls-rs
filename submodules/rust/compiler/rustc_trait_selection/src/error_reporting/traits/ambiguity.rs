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
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_errors :: { Applicability , Diag , E0283 , E0284 , E0790 , MultiSpan , struct_span_code_err } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: { CRATE_DEF_ID , DefId } ;}
mkuse!{use rustc_hir :: intravisit :: Visitor as _ ;}
mkuse!{use rustc_infer :: infer :: { BoundRegionConversionTime , InferCtxt } ;}
mkuse!{use rustc_infer :: traits :: util :: elaborate ;}
mkuse!{use rustc_infer :: traits :: { Obligation , ObligationCause , ObligationCauseCode , PolyTraitObligation , PredicateObligation , } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeVisitable as _ , TypeVisitableExt as _ } ;}
mkuse!{use rustc_session :: parse :: feature_err_unstable_feature_bound ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed , Span } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: error_reporting :: TypeErrCtxt ;}
mkuse!{use crate :: error_reporting :: infer :: need_type_info :: TypeAnnotationNeeded ;}
mkuse!{use crate :: error_reporting :: traits :: { FindExprBySpan , to_pretty_impl_header } ;}
mkuse!{use crate :: traits :: ObligationCtxt ;}
mkuse!{use crate :: traits :: query :: evaluate_obligation :: InferCtxtExt ;}
mkitem!{mkenum!{#[derive (Debug)] pub enum CandidateSource { DefId (DefId) , ParamEnv (Span) , }}}

macro_rules! compute_applicable_impls_for_diagnostics_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_applicable_impls_for_diagnostics in module {}", module_path!());
    };
}

mkfn!{
    compute_applicable_impls_for_diagnostics_introspect!();
    pub fn compute_applicable_impls_for_diagnostics < 'tcx > (infcx : & InferCtxt < 'tcx > , obligation : & PolyTraitObligation < 'tcx > ,) -> Vec < CandidateSource > { let tcx = infcx . tcx ; let param_env = obligation . param_env ; let predicate_polarity = obligation . predicate . skip_binder () . polarity ; let impl_may_apply = | impl_def_id | { let ocx = ObligationCtxt :: new (infcx) ; infcx . enter_forall (obligation . predicate , | placeholder_obligation | { let obligation_trait_ref = ocx . normalize (& ObligationCause :: dummy () , param_env , placeholder_obligation . trait_ref ,) ; let impl_args = infcx . fresh_args_for_item (DUMMY_SP , impl_def_id) ; let impl_trait_ref = tcx . impl_trait_ref (impl_def_id) . unwrap () . instantiate (tcx , impl_args) ; let impl_trait_ref = ocx . normalize (& ObligationCause :: dummy () , param_env , impl_trait_ref) ; if let Err (_) = ocx . eq (& ObligationCause :: dummy () , param_env , obligation_trait_ref , impl_trait_ref) { return false ; } let impl_trait_header = tcx . impl_trait_header (impl_def_id) . unwrap () ; let impl_polarity = impl_trait_header . polarity ; match (impl_polarity , predicate_polarity) { (ty :: ImplPolarity :: Positive , ty :: PredicatePolarity :: Positive) | (ty :: ImplPolarity :: Negative , ty :: PredicatePolarity :: Negative) => { } _ => return false , } let obligations = tcx . predicates_of (impl_def_id) . instantiate (tcx , impl_args) . into_iter () . map (| (predicate , _) | { Obligation :: new (tcx , ObligationCause :: dummy () , param_env , predicate) }) . filter (| obligation | { infcx . next_trait_solver () || infcx . evaluate_obligation (obligation) . is_ok () }) ; ocx . register_obligations (obligations) ; ocx . select_where_possible () . is_empty () }) } ; let param_env_candidate_may_apply = | poly_trait_predicate : ty :: PolyTraitPredicate < 'tcx > | { let ocx = ObligationCtxt :: new (infcx) ; infcx . enter_forall (obligation . predicate , | placeholder_obligation | { let obligation_trait_ref = ocx . normalize (& ObligationCause :: dummy () , param_env , placeholder_obligation . trait_ref ,) ; let param_env_predicate = infcx . instantiate_binder_with_fresh_vars (DUMMY_SP , BoundRegionConversionTime :: HigherRankedType , poly_trait_predicate ,) ; let param_env_trait_ref = ocx . normalize (& ObligationCause :: dummy () , param_env , param_env_predicate . trait_ref) ; if let Err (_) = ocx . eq (& ObligationCause :: dummy () , param_env , obligation_trait_ref , param_env_trait_ref ,) { return false ; } ocx . select_where_possible () . is_empty () }) } ; let mut ambiguities = Vec :: new () ; tcx . for_each_relevant_impl (obligation . predicate . def_id () , obligation . predicate . skip_binder () . trait_ref . self_ty () , | impl_def_id | { if infcx . probe (| _ | impl_may_apply (impl_def_id)) { ambiguities . push (CandidateSource :: DefId (impl_def_id)) } } ,) ; let body_id = obligation . cause . body_id ; if body_id != CRATE_DEF_ID { let predicates = tcx . predicates_of (body_id . to_def_id ()) . instantiate_identity (tcx) ; for (pred , span) in elaborate (tcx , predicates . into_iter ()) { let kind = pred . kind () ; if let ty :: ClauseKind :: Trait (trait_pred) = kind . skip_binder () && param_env_candidate_may_apply (kind . rebind (trait_pred)) { if kind . rebind (trait_pred . trait_ref) == ty :: Binder :: dummy (ty :: TraitRef :: identity (tcx , trait_pred . def_id ())) { ambiguities . push (CandidateSource :: ParamEnv (tcx . def_span (trait_pred . def_id ()))) } else { ambiguities . push (CandidateSource :: ParamEnv (span)) } } } } ambiguities }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > TypeErrCtxt < 'a , 'tcx > { #[instrument (skip (self) , level = "debug")] pub (super) fn maybe_report_ambiguity (& self , obligation : & PredicateObligation < 'tcx > ,) -> ErrorGuaranteed { let predicate = self . resolve_vars_if_possible (obligation . predicate) ; let span = obligation . cause . span ; let mut long_ty_path = None ; debug ! (? predicate , obligation . cause . code = ? obligation . cause . code ()) ; let bound_predicate = predicate . kind () ; let mut err = match bound_predicate . skip_binder () { ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (data)) => { let trait_pred = bound_predicate . rebind (data) ; debug ! (? trait_pred) ; if let Err (e) = predicate . error_reported () { return e ; } if let Err (guar) = self . tcx . ensure_ok () . coherent_trait (trait_pred . def_id ()) { return guar ; } if matches ! (self . tcx . as_lang_item (trait_pred . def_id ()) , Some (LangItem :: Sized | LangItem :: MetaSized)) { return match self . tainted_by_errors () { None => self . emit_inference_failure_err (obligation . cause . body_id , span , trait_pred . self_ty () . skip_binder () . into () , TypeAnnotationNeeded :: E0282 , false ,) . emit () , Some (e) => e , } ; } let term = data . trait_ref . args . iter () . filter_map (ty :: GenericArg :: as_term) . find (| s | s . has_non_region_infer ()) ; let mut err = if let Some (term) = term { self . emit_inference_failure_err (obligation . cause . body_id , span , term , TypeAnnotationNeeded :: E0283 , true ,) } else { struct_span_code_err ! (self . dcx () , span , E0283 , "type annotations needed: cannot satisfy `{}`" , self . tcx . short_string (predicate , & mut long_ty_path) ,) . with_long_ty_path (long_ty_path) } ; let mut ambiguities = compute_applicable_impls_for_diagnostics (self . infcx , & obligation . with (self . tcx , trait_pred) ,) ; let has_non_region_infer = trait_pred . skip_binder () . trait_ref . args . types () . any (| t | ! t . is_ty_or_numeric_infer ()) ; if ambiguities . len () > 5 { let infcx = self . infcx ; if ! ambiguities . iter () . all (| option | match option { CandidateSource :: DefId (did) => infcx . tcx . generics_of (* did) . count () == 0 , CandidateSource :: ParamEnv (_) => true , }) { ambiguities . retain (| option | match option { CandidateSource :: DefId (did) => infcx . tcx . generics_of (* did) . count () == 0 , CandidateSource :: ParamEnv (_) => true , }) ; } } if ambiguities . len () > 1 && ambiguities . len () < 10 && has_non_region_infer { if let Some (e) = self . tainted_by_errors () && term . is_none () { err . cancel () ; return e ; } self . annotate_source_of_ambiguity (& mut err , & ambiguities , predicate) ; } else { if let Some (e) = self . tainted_by_errors () { err . cancel () ; return e ; } let pred = self . tcx . short_string (predicate , & mut err . long_ty_path ()) ; err . note (format ! ("cannot satisfy `{pred}`")) ; let impl_candidates = self . find_similar_impl_candidates (predicate . as_trait_clause () . unwrap ()) ; if impl_candidates . len () < 40 { self . report_similar_impl_candidates (impl_candidates . as_slice () , trait_pred , obligation . cause . body_id , & mut err , false , obligation . param_env ,) ; } } if let ObligationCauseCode :: WhereClause (def_id , _) | ObligationCauseCode :: WhereClauseInExpr (def_id , ..) = * obligation . cause . code () { self . suggest_fully_qualified_path (& mut err , def_id , span , trait_pred . def_id ()) ; } if term . is_some_and (| term | term . as_type () . is_some ()) && let Some (body) = self . tcx . hir_maybe_body_owned_by (obligation . cause . body_id) { let mut expr_finder = FindExprBySpan :: new (span , self . tcx) ; expr_finder . visit_expr (& body . value) ; if let Some (hir :: Expr { kind : hir :: ExprKind :: Call (hir :: Expr { kind : hir :: ExprKind :: Path (hir :: QPath :: Resolved (None , path)) , .. } , _ ,) | hir :: ExprKind :: Path (hir :: QPath :: Resolved (None , path)) , .. }) = expr_finder . result && let [.. , trait_path_segment @ hir :: PathSegment { res : Res :: Def (DefKind :: Trait , trait_id) , .. } , hir :: PathSegment { ident : assoc_item_ident , res : Res :: Def (_ , item_id) , .. } ,] = path . segments && data . trait_ref . def_id == * trait_id && self . tcx . trait_of_assoc (* item_id) == Some (* trait_id) && let None = self . tainted_by_errors () { let assoc_item = self . tcx . associated_item (item_id) ; let (verb , noun) = match assoc_item . kind { ty :: AssocKind :: Const { .. } => ("refer to the" , "constant") , ty :: AssocKind :: Fn { .. } => ("call" , "function") , ty :: AssocKind :: Type { .. } => ("refer to the" , "type") , } ; err . cancel () ; err = self . dcx () . struct_span_err (span , format ! ("cannot {verb} associated {noun} on trait without specifying the \
                                 corresponding `impl` type" ,) ,) ; err . code (E0790) ; if item_id . is_local () { let trait_ident = self . tcx . item_name (* trait_id) ; err . span_label (self . tcx . def_span (* item_id) , format ! ("`{trait_ident}::{assoc_item_ident}` defined here") ,) ; } err . span_label (span , format ! ("cannot {verb} associated {noun} of trait")) ; let trait_impls = self . tcx . trait_impls_of (data . trait_ref . def_id) ; if let Some (impl_def_id) = trait_impls . non_blanket_impls () . values () . flatten () . next () { let non_blanket_impl_count = trait_impls . non_blanket_impls () . values () . flatten () . count () ; let (message , self_types) = if non_blanket_impl_count == 1 { ("use the fully-qualified path to the only available \
                                     implementation" , vec ! [format ! ("{}" , self . tcx . type_of (impl_def_id) . instantiate_identity ())] ,) } else if non_blanket_impl_count < 20 { ("use a fully-qualified path to one of the available \
                                     implementations" , trait_impls . non_blanket_impls () . values () . flatten () . map (| id | { format ! ("{}" , self . tcx . type_of (id) . instantiate_identity ()) }) . collect :: < Vec < String > > () ,) } else { ("use a fully-qualified path to a specific available \
                                     implementation" , vec ! ["/* self type */" . to_string ()] ,) } ; let suggestions : Vec < _ > = self_types . into_iter () . map (| self_type | { let mut suggestions = vec ! [(path . span . shrink_to_lo () , format ! ("<{self_type} as ") ,)] ; if let Some (generic_arg) = trait_path_segment . args { let between_span = trait_path_segment . ident . span . between (generic_arg . span_ext) ; suggestions . push ((between_span , "" . to_string ())) ; suggestions . push ((generic_arg . span_ext . shrink_to_hi () , ">" . to_string () ,)) ; } else { suggestions . push ((trait_path_segment . ident . span . shrink_to_hi () , ">" . to_string () ,)) ; } suggestions }) . collect () ; err . multipart_suggestions (message , suggestions , Applicability :: MaybeIncorrect ,) ; } } } ; err } ty :: PredicateKind :: Clause (ty :: ClauseKind :: WellFormed (term)) => { if let Err (e) = term . error_reported () { return e ; } if let Some (e) = self . tainted_by_errors () { return e ; } self . emit_inference_failure_err (obligation . cause . body_id , span , term , TypeAnnotationNeeded :: E0282 , false ,) } ty :: PredicateKind :: Subtype (data) => { if let Err (e) = data . error_reported () { return e ; } if let Some (e) = self . tainted_by_errors () { return e ; } let ty :: SubtypePredicate { a_is_expected : _ , a , b } = data ; assert ! (a . is_ty_var () && b . is_ty_var ()) ; self . emit_inference_failure_err (obligation . cause . body_id , span , a . into () , TypeAnnotationNeeded :: E0282 , true ,) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: Projection (data)) => { if let Err (e) = predicate . error_reported () { return e ; } if let Some (e) = self . tainted_by_errors () { return e ; } if let Err (guar) = self . tcx . ensure_ok () . coherent_trait (self . tcx . parent (data . projection_term . def_id)) { return guar ; } let term = data . projection_term . args . iter () . filter_map (ty :: GenericArg :: as_term) . chain ([data . term]) . find (| g | g . has_non_region_infer ()) ; let predicate = self . tcx . short_string (predicate , & mut long_ty_path) ; if let Some (term) = term { self . emit_inference_failure_err (obligation . cause . body_id , span , term , TypeAnnotationNeeded :: E0284 , true ,) . with_note (format ! ("cannot satisfy `{predicate}`")) . with_long_ty_path (long_ty_path) } else { struct_span_code_err ! (self . dcx () , span , E0284 , "type annotations needed: cannot satisfy `{predicate}`" ,) . with_span_label (span , format ! ("cannot satisfy `{predicate}`")) . with_long_ty_path (long_ty_path) } } ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstEvaluatable (data)) => { if let Err (e) = predicate . error_reported () { return e ; } if let Some (e) = self . tainted_by_errors () { return e ; } let term = data . walk () . filter_map (ty :: GenericArg :: as_term) . find (| term | term . is_infer ()) ; if let Some (term) = term { self . emit_inference_failure_err (obligation . cause . body_id , span , term , TypeAnnotationNeeded :: E0284 , true ,) } else { let predicate = self . tcx . short_string (predicate , & mut long_ty_path) ; struct_span_code_err ! (self . dcx () , span , E0284 , "type annotations needed: cannot satisfy `{predicate}`" ,) . with_span_label (span , format ! ("cannot satisfy `{predicate}`")) . with_long_ty_path (long_ty_path) } } ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstArgHasType (ct , ..)) => self . emit_inference_failure_err (obligation . cause . body_id , span , ct . into () , TypeAnnotationNeeded :: E0284 , true ,) , ty :: PredicateKind :: NormalizesTo (ty :: NormalizesTo { alias , term }) if term . is_infer () => { if let Some (e) = self . tainted_by_errors () { return e ; } let alias = self . tcx . short_string (alias , & mut long_ty_path) ; struct_span_code_err ! (self . dcx () , span , E0284 , "type annotations needed: cannot normalize `{alias}`" ,) . with_span_label (span , format ! ("cannot normalize `{alias}`")) . with_long_ty_path (long_ty_path) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: UnstableFeature (sym)) => { if let Some (e) = self . tainted_by_errors () { return e ; } if self . tcx . features () . staged_api () { self . dcx () . struct_span_err (span , format ! ("unstable feature `{sym}` is used without being enabled.") ,) . with_help (format ! ("The feature can be enabled by marking the current item with `#[unstable_feature_bound({sym})]`")) } else { feature_err_unstable_feature_bound (& self . tcx . sess , sym , span , format ! ("use of unstable library feature `{sym}`") ,) } } _ => { if let Some (e) = self . tainted_by_errors () { return e ; } let predicate = self . tcx . short_string (predicate , & mut long_ty_path) ; struct_span_code_err ! (self . dcx () , span , E0284 , "type annotations needed: cannot satisfy `{predicate}`" ,) . with_span_label (span , format ! ("cannot satisfy `{predicate}`")) . with_long_ty_path (long_ty_path) } } ; self . note_obligation_cause (& mut err , obligation) ; err . emit () } fn annotate_source_of_ambiguity (& self , err : & mut Diag < '_ > , ambiguities : & [CandidateSource] , predicate : ty :: Predicate < 'tcx > ,) { let mut spans = vec ! [] ; let mut crates = vec ! [] ; let mut post = vec ! [] ; let mut has_param_env = false ; for ambiguity in ambiguities { match ambiguity { CandidateSource :: DefId (impl_def_id) => match self . tcx . span_of_impl (* impl_def_id) { Ok (span) => spans . push (span) , Err (name) => { crates . push (name) ; if let Some (header) = to_pretty_impl_header (self . tcx , * impl_def_id) { post . push (header) ; } } } , CandidateSource :: ParamEnv (span) => { has_param_env = true ; spans . push (* span) ; } } } let mut crate_names : Vec < _ > = crates . iter () . map (| n | format ! ("`{n}`")) . collect () ; crate_names . sort () ; crate_names . dedup () ; post . sort () ; post . dedup () ; if self . tainted_by_errors () . is_some () && (crate_names . len () == 1 && spans . len () == 0 && ["`core`" , "`alloc`" , "`std`"] . contains (& crate_names [0] . as_str ()) || predicate . visit_with (& mut HasNumericInferVisitor) . is_break ()) { err . downgrade_to_delayed_bug () ; return ; } let msg = format ! ("multiple `impl`s{} satisfying `{}` found" , if has_param_env { " or `where` clauses" } else { "" } , predicate) ; let post = if post . len () > 1 || (post . len () == 1 && post [0] . contains ('\n')) { format ! (":\n{}" , post . iter () . map (| p | format ! ("- {p}")) . collect ::< Vec < _ >> () . join ("\n") ,) } else if post . len () == 1 { format ! (": `{}`" , post [0]) } else { String :: new () } ; match (spans . len () , crates . len () , crate_names . len ()) { (0 , 0 , 0) => { err . note (format ! ("cannot satisfy `{predicate}`")) ; } (0 , _ , 1) => { err . note (format ! ("{} in the `{}` crate{}" , msg , crates [0] , post ,)) ; } (0 , _ , _) => { err . note (format ! ("{} in the following crates: {}{}" , msg , crate_names . join (", ") , post ,)) ; } (_ , 0 , 0) => { let span : MultiSpan = spans . into () ; err . span_note (span , msg) ; } (_ , 1 , 1) => { let span : MultiSpan = spans . into () ; err . span_note (span , msg) ; err . note (format ! ("and another `impl` found in the `{}` crate{}" , crates [0] , post ,)) ; } _ => { let span : MultiSpan = spans . into () ; err . span_note (span , msg) ; err . note (format ! ("and more `impl`s found in the following crates: {}{}" , crate_names . join (", ") , post ,)) ; } } } }}}
mkitem!{mkstruct!{struct HasNumericInferVisitor ;}}
mkitem!{mkimpl!{impl < 'tcx > ty :: TypeVisitor < TyCtxt < 'tcx > > for HasNumericInferVisitor { type Result = ControlFlow < () > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { if matches ! (ty . kind () , ty :: Infer (ty :: FloatVar (_) | ty :: IntVar (_))) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } }}}