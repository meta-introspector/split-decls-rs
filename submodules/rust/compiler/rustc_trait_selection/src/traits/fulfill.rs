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
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use rustc_data_structures :: obligation_forest :: { Error , ForestObligation , ObligationForest , ObligationProcessor , Outcome , ProcessResult , } ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_infer :: infer :: DefineOpaqueTypes ;}
mkuse!{use rustc_infer :: traits :: { FromSolverError , PolyTraitObligation , PredicateObligations , ProjectionCacheKey , SelectionError , TraitEngine , } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: abstract_const :: NotConstEvaluatable ;}
mkuse!{use rustc_middle :: ty :: error :: { ExpectedFound , TypeError } ;}
mkuse!{use rustc_middle :: ty :: { self , Binder , Const , GenericArgsRef , TypeVisitable , TypeVisitableExt , TypingMode , may_use_unstable_feature , } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use thin_vec :: { ThinVec , thin_vec } ;}
mkuse!{use tracing :: { debug , debug_span , instrument } ;}
mkuse!{use super :: effects :: { self , HostEffectObligation } ;}
mkuse!{use super :: project :: { self , ProjectAndUnifyResult } ;}
mkuse!{use super :: select :: SelectionContext ;}
mkuse!{use super :: { EvaluationResult , FulfillmentError , FulfillmentErrorCode , PredicateObligation , ScrubbedTraitError , const_evaluatable , wf , } ;}
mkuse!{use crate :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use crate :: infer :: { InferCtxt , TyOrConstInferVar } ;}
mkuse!{use crate :: solve :: StalledOnCoroutines ;}
mkuse!{use crate :: traits :: normalize :: normalize_with_depth_to ;}
mkuse!{use crate :: traits :: project :: { PolyProjectionObligation , ProjectionCacheKeyExt as _ } ;}
mkuse!{use crate :: traits :: query :: evaluate_obligation :: InferCtxtExt ;}
mkuse!{use crate :: traits :: { EvaluateConstErr , sizedness_fast_path } ;}
mkitem!{pub (crate) type PendingPredicateObligations < 'tcx > = ThinVec < PendingPredicateObligation < 'tcx > > ;}
mkitem!{mkimpl!{impl < 'tcx > ForestObligation for PendingPredicateObligation < 'tcx > { #[doc = " Note that we include both the `ParamEnv` and the `Predicate`,"] #[doc = " as the `ParamEnv` can influence whether fulfillment succeeds"] #[doc = " or fails."] type CacheKey = ty :: ParamEnvAnd < 'tcx , ty :: Predicate < 'tcx > > ; fn as_cache_key (& self) -> Self :: CacheKey { self . obligation . param_env . and (self . obligation . predicate) } }}}
mkitem!{mkstruct!{#[doc = " The fulfillment context is used to drive trait resolution. It"] #[doc = " consists of a list of obligations that must be (eventually)"] #[doc = " satisfied. The job is to track which are satisfied, which yielded"] #[doc = " errors, and which are still pending. At any point, users can call"] #[doc = " `select_where_possible`, and the fulfillment context will try to do"] #[doc = " selection, retaining only those obligations that remain"] #[doc = " ambiguous. This may be helpful in pushing type inference"] #[doc = " along. Once all type inference constraints have been generated, the"] #[doc = " method `select_all_or_error` can be used to report any remaining"] #[doc = " ambiguous cases as errors."] pub struct FulfillmentContext < 'tcx , E : 'tcx > { #[doc = " A list of all obligations that have been registered with this"] #[doc = " fulfillment context."] predicates : ObligationForest < PendingPredicateObligation < 'tcx > > , #[doc = " The snapshot in which this context was created. Using the context"] #[doc = " outside of this snapshot leads to subtle bugs if the snapshot"] #[doc = " gets rolled back. Because of this we explicitly check that we only"] #[doc = " use the context in exactly this snapshot."] usable_in_snapshot : usize , _errors : PhantomData < E > , }}}
mkitem!{mkstruct!{#[derive (Clone , Debug)] pub struct PendingPredicateObligation < 'tcx > { pub obligation : PredicateObligation < 'tcx > , pub stalled_on : Vec < TyOrConstInferVar > , }}}
mkitem!{#[cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (PendingPredicateObligation <'_ >, 72) ;}
mkitem!{mkimpl!{impl < 'tcx , E > FulfillmentContext < 'tcx , E > where E : FromSolverError < 'tcx , OldSolverError < 'tcx > > , { #[doc = " Creates a new fulfillment context."] pub (super) fn new (infcx : & InferCtxt < 'tcx >) -> FulfillmentContext < 'tcx , E > { assert ! (! infcx . next_trait_solver () , "old trait solver fulfillment context created when \
            infcx is set up for new trait solver") ; FulfillmentContext { predicates : ObligationForest :: new () , usable_in_snapshot : infcx . num_open_snapshots () , _errors : PhantomData , } } #[doc = " Attempts to select obligations using `selcx`."] fn select (& mut self , selcx : SelectionContext < '_ , 'tcx >) -> Vec < E > { let span = debug_span ! ("select" , obligation_forest_size = ? self . predicates . len ()) ; let _enter = span . enter () ; let infcx = selcx . infcx ; let outcome : Outcome < _ , _ > = self . predicates . process_obligations (& mut FulfillProcessor { selcx }) ; let errors : Vec < E > = outcome . errors . into_iter () . map (| err | E :: from_solver_error (infcx , OldSolverError (err))) . collect () ; debug ! ("select({} predicates remaining, {} errors) done" , self . predicates . len () , errors . len ()) ; errors } }}}
mkitem!{mkimpl!{impl < 'tcx , E > TraitEngine < 'tcx , E > for FulfillmentContext < 'tcx , E > where E : FromSolverError < 'tcx , OldSolverError < 'tcx > > , { #[inline] fn register_predicate_obligation (& mut self , infcx : & InferCtxt < 'tcx > , mut obligation : PredicateObligation < 'tcx > ,) { assert_eq ! (self . usable_in_snapshot , infcx . num_open_snapshots ()) ; debug_assert ! (! obligation . param_env . has_non_region_infer ()) ; obligation . predicate = infcx . resolve_vars_if_possible (obligation . predicate) ; debug ! (? obligation , "register_predicate_obligation") ; self . predicates . register_obligation (PendingPredicateObligation { obligation , stalled_on : vec ! [] }) ; } fn collect_remaining_errors (& mut self , infcx : & InferCtxt < 'tcx >) -> Vec < E > { self . predicates . to_errors (FulfillmentErrorCode :: Ambiguity { overflow : None }) . into_iter () . map (| err | E :: from_solver_error (infcx , OldSolverError (err))) . collect () } fn select_where_possible (& mut self , infcx : & InferCtxt < 'tcx >) -> Vec < E > { let selcx = SelectionContext :: new (infcx) ; self . select (selcx) } fn drain_stalled_obligations_for_coroutines (& mut self , infcx : & InferCtxt < 'tcx > ,) -> PredicateObligations < 'tcx > { let stalled_coroutines = match infcx . typing_mode () { TypingMode :: Analysis { defining_opaque_types_and_generators } => { defining_opaque_types_and_generators } TypingMode :: Coherence | TypingMode :: Borrowck { defining_opaque_types : _ } | TypingMode :: PostBorrowckAnalysis { defined_opaque_types : _ } | TypingMode :: PostAnalysis => return Default :: default () , } ; if stalled_coroutines . is_empty () { return Default :: default () ; } let mut processor = DrainProcessor { infcx , removed_predicates : PredicateObligations :: new () , stalled_coroutines , } ; let outcome : Outcome < _ , _ > = self . predicates . process_obligations (& mut processor) ; assert ! (outcome . errors . is_empty ()) ; return processor . removed_predicates ; struct DrainProcessor < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , removed_predicates : PredicateObligations < 'tcx > , stalled_coroutines : & 'tcx ty :: List < LocalDefId > , } impl < 'tcx > ObligationProcessor for DrainProcessor < '_ , 'tcx > { type Obligation = PendingPredicateObligation < 'tcx > ; type Error = ! ; type OUT = Outcome < Self :: Obligation , Self :: Error > ; fn needs_process_obligation (& self , pending_obligation : & Self :: Obligation) -> bool { self . infcx . resolve_vars_if_possible (pending_obligation . obligation . predicate) . visit_with (& mut StalledOnCoroutines { stalled_coroutines : self . stalled_coroutines , span : DUMMY_SP , cache : Default :: default () , }) . is_break () } fn process_obligation (& mut self , pending_obligation : & mut PendingPredicateObligation < 'tcx > ,) -> ProcessResult < PendingPredicateObligation < 'tcx > , ! > { assert ! (self . needs_process_obligation (pending_obligation)) ; self . removed_predicates . push (pending_obligation . obligation . clone ()) ; ProcessResult :: Changed (Default :: default ()) } fn process_backedge < 'c , I > (& mut self , cycle : I , _marker : PhantomData < & 'c PendingPredicateObligation < 'tcx > > ,) -> Result < () , ! > where I : Clone + Iterator < Item = & 'c PendingPredicateObligation < 'tcx > > , { self . removed_predicates . extend (cycle . map (| c | c . obligation . clone ())) ; Ok (()) } } } fn has_pending_obligations (& self) -> bool { self . predicates . has_pending_obligations () } fn pending_obligations (& self) -> PredicateObligations < 'tcx > { self . predicates . map_pending_obligations (| o | o . obligation . clone ()) } }}}
mkitem!{mkstruct!{struct FulfillProcessor < 'a , 'tcx > { selcx : SelectionContext < 'a , 'tcx > , }}}

macro_rules! mk_pending_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mk_pending in module {}", module_path!());
    };
}

mkfn!{
    mk_pending_introspect!();
    fn mk_pending < 'tcx > (parent : & PredicateObligation < 'tcx > , os : PredicateObligations < 'tcx > ,) -> PendingPredicateObligations < 'tcx > { os . into_iter () . map (| mut o | { o . set_depth_from_parent (parent . recursion_depth) ; PendingPredicateObligation { obligation : o , stalled_on : vec ! [] } }) . collect () }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > ObligationProcessor for FulfillProcessor < 'a , 'tcx > { type Obligation = PendingPredicateObligation < 'tcx > ; type Error = FulfillmentErrorCode < 'tcx > ; type OUT = Outcome < Self :: Obligation , Self :: Error > ; #[doc = " Compared to `needs_process_obligation` this and its callees"] #[doc = " contain some optimizations that come at the price of false negatives."] #[doc = ""] #[doc = " They"] #[doc = " - reduce branching by covering only the most common case"] #[doc = " - take a read-only view of the unification tables which allows skipping undo_log"] #[doc = "   construction."] #[doc = " - bail out on value-cache misses in ena to avoid pointer chasing"] #[doc = " - hoist RefCell locking out of the loop"] #[inline] fn skippable_obligations < 'b > (& 'b self , it : impl Iterator < Item = & 'b Self :: Obligation > ,) -> usize { let is_unchanged = self . selcx . infcx . is_ty_infer_var_definitely_unchanged () ; it . take_while (| o | match o . stalled_on . as_slice () { [o] => is_unchanged (* o) , _ => false , }) . count () } #[doc = " Identifies whether a predicate obligation needs processing."] #[doc = ""] #[doc = " This is always inlined because it has a single callsite and it is"] #[doc = " called *very* frequently. Be careful modifying this code! Several"] #[doc = " compile-time benchmarks are very sensitive to even small changes."] #[inline (always)] fn needs_process_obligation (& self , pending_obligation : & Self :: Obligation) -> bool { let stalled_on = & pending_obligation . stalled_on ; match stalled_on . len () { 1 => self . selcx . infcx . ty_or_const_infer_var_changed (stalled_on [0]) , 0 => true , _ => (| | { for & infer_var in stalled_on { if self . selcx . infcx . ty_or_const_infer_var_changed (infer_var) { return true ; } } false }) () , } } #[doc = " Processes a predicate obligation and returns either:"] #[doc = " - `Changed(v)` if the predicate is true, presuming that `v` are also true"] #[doc = " - `Unchanged` if we don't have enough info to be sure"] #[doc = " - `Error(e)` if the predicate does not hold"] #[doc = ""] #[doc = " This is called much less often than `needs_process_obligation`, so we"] #[doc = " never inline it."] #[inline (never)] #[instrument (level = "debug" , skip (self , pending_obligation))] fn process_obligation (& mut self , pending_obligation : & mut PendingPredicateObligation < 'tcx > ,) -> ProcessResult < PendingPredicateObligation < 'tcx > , FulfillmentErrorCode < 'tcx > > { pending_obligation . stalled_on . truncate (0) ; let obligation = & mut pending_obligation . obligation ; debug ! (? obligation , "pre-resolve") ; if obligation . predicate . has_non_region_infer () { obligation . predicate = self . selcx . infcx . resolve_vars_if_possible (obligation . predicate) ; } let obligation = & pending_obligation . obligation ; let infcx = self . selcx . infcx ; if sizedness_fast_path (infcx . tcx , obligation . predicate , obligation . param_env) { return ProcessResult :: Changed (thin_vec ! []) ; } if obligation . predicate . has_aliases () { let mut obligations = PredicateObligations :: new () ; let predicate = normalize_with_depth_to (& mut self . selcx , obligation . param_env , obligation . cause . clone () , obligation . recursion_depth + 1 , obligation . predicate , & mut obligations ,) ; if predicate != obligation . predicate { obligations . push (obligation . with (infcx . tcx , predicate)) ; return ProcessResult :: Changed (mk_pending (obligation , obligations)) ; } } let binder = obligation . predicate . kind () ; match binder . no_bound_vars () { None => match binder . skip_binder () { ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (trait_ref)) => { let trait_obligation = obligation . with (infcx . tcx , binder . rebind (trait_ref)) ; self . process_trait_obligation (obligation , trait_obligation , & mut pending_obligation . stalled_on ,) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: Projection (data)) => { let project_obligation = obligation . with (infcx . tcx , binder . rebind (data)) ; self . process_projection_obligation (obligation , project_obligation , & mut pending_obligation . stalled_on ,) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: RegionOutlives (_)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: TypeOutlives (_)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstArgHasType (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: WellFormed (_)) | ty :: PredicateKind :: DynCompatible (_) | ty :: PredicateKind :: Subtype (_) | ty :: PredicateKind :: Coerce (_) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstEvaluatable (..)) | ty :: PredicateKind :: ConstEquate (..) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: HostEffect (..)) => { let pred = ty :: Binder :: dummy (infcx . enter_forall_and_leak_universe (binder)) ; let mut obligations = PredicateObligations :: with_capacity (1) ; obligations . push (obligation . with (infcx . tcx , pred)) ; ProcessResult :: Changed (mk_pending (obligation , obligations)) } ty :: PredicateKind :: Ambiguous => ProcessResult :: Unchanged , ty :: PredicateKind :: NormalizesTo (..) => { bug ! ("NormalizesTo is only used by the new solver") } ty :: PredicateKind :: AliasRelate (..) => { bug ! ("AliasRelate is only used by the new solver") } ty :: PredicateKind :: Clause (ty :: ClauseKind :: UnstableFeature (_)) => { unreachable ! ("unexpected higher ranked `UnstableFeature` goal") } } , Some (pred) => match pred { ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (data)) => { let trait_obligation = obligation . with (infcx . tcx , Binder :: dummy (data)) ; self . process_trait_obligation (obligation , trait_obligation , & mut pending_obligation . stalled_on ,) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: HostEffect (data)) => { let host_obligation = obligation . with (infcx . tcx , data) ; self . process_host_obligation (obligation , host_obligation , & mut pending_obligation . stalled_on ,) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: RegionOutlives (data)) => { if infcx . considering_regions { infcx . register_region_outlives_constraint (data , & obligation . cause) ; } ProcessResult :: Changed (Default :: default ()) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (t_a , r_b ,))) => { if infcx . considering_regions { infcx . register_type_outlives_constraint (t_a , r_b , & obligation . cause) ; } ProcessResult :: Changed (Default :: default ()) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: Projection (ref data)) => { let project_obligation = obligation . with (infcx . tcx , Binder :: dummy (* data)) ; self . process_projection_obligation (obligation , project_obligation , & mut pending_obligation . stalled_on ,) } ty :: PredicateKind :: DynCompatible (trait_def_id) => { if ! self . selcx . tcx () . is_dyn_compatible (trait_def_id) { ProcessResult :: Error (FulfillmentErrorCode :: Select (SelectionError :: Unimplemented ,)) } else { ProcessResult :: Changed (Default :: default ()) } } ty :: PredicateKind :: Ambiguous => ProcessResult :: Unchanged , ty :: PredicateKind :: NormalizesTo (..) => { bug ! ("NormalizesTo is only used by the new solver") } ty :: PredicateKind :: AliasRelate (..) => { bug ! ("AliasRelate is only used by the new solver") } ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstArgHasType (ct , ty)) => { let ct = infcx . shallow_resolve_const (ct) ; let ct_ty = match ct . kind () { ty :: ConstKind :: Infer (var) => { let var = match var { ty :: InferConst :: Var (vid) => TyOrConstInferVar :: Const (vid) , ty :: InferConst :: Fresh (_) => { bug ! ("encountered fresh const in fulfill") } } ; pending_obligation . stalled_on . clear () ; pending_obligation . stalled_on . extend ([var]) ; return ProcessResult :: Unchanged ; } ty :: ConstKind :: Error (_) => { return ProcessResult :: Changed (PendingPredicateObligations :: new ()) ; } ty :: ConstKind :: Value (cv) => cv . ty , ty :: ConstKind :: Unevaluated (uv) => { infcx . tcx . type_of (uv . def) . instantiate (infcx . tcx , uv . args) } ty :: ConstKind :: Expr (_) => { return ProcessResult :: Changed (mk_pending (obligation , PredicateObligations :: new () ,)) ; } ty :: ConstKind :: Placeholder (_) => { bug ! ("placeholder const {:?} in old solver" , ct) } ty :: ConstKind :: Bound (_ , _) => bug ! ("escaping bound vars in {:?}" , ct) , ty :: ConstKind :: Param (param_ct) => { param_ct . find_const_ty_from_env (obligation . param_env) } } ; match infcx . at (& obligation . cause , obligation . param_env) . eq (DefineOpaqueTypes :: Yes , ct_ty , ty ,) { Ok (inf_ok) => ProcessResult :: Changed (mk_pending (obligation , inf_ok . into_obligations () ,)) , Err (_) => ProcessResult :: Error (FulfillmentErrorCode :: Select (SelectionError :: ConstArgHasWrongType { ct , ct_ty , expected_ty : ty } ,)) , } } _ if ! self . selcx . tcx () . recursion_limit () . value_within_limit (obligation . recursion_depth) => { self . selcx . infcx . err_ctxt () . report_overflow_obligation (& obligation , false) ; } ty :: PredicateKind :: Clause (ty :: ClauseKind :: WellFormed (term)) => { if term . is_trivially_wf (self . selcx . tcx ()) { return ProcessResult :: Changed (thin_vec ! []) ; } match wf :: obligations (self . selcx . infcx , obligation . param_env , obligation . cause . body_id , obligation . recursion_depth + 1 , term , obligation . cause . span ,) { None => { pending_obligation . stalled_on = vec ! [TyOrConstInferVar :: maybe_from_term (term) . unwrap ()] ; ProcessResult :: Unchanged } Some (os) => ProcessResult :: Changed (mk_pending (obligation , os)) , } } ty :: PredicateKind :: Subtype (subtype) => { match self . selcx . infcx . subtype_predicate (& obligation . cause , obligation . param_env , Binder :: dummy (subtype) ,) { Err ((a , b)) => { pending_obligation . stalled_on = vec ! [TyOrConstInferVar :: Ty (a) , TyOrConstInferVar :: Ty (b)] ; ProcessResult :: Unchanged } Ok (Ok (ok)) => { ProcessResult :: Changed (mk_pending (obligation , ok . obligations)) } Ok (Err (err)) => { let expected_found = if subtype . a_is_expected { ExpectedFound :: new (subtype . a , subtype . b) } else { ExpectedFound :: new (subtype . b , subtype . a) } ; ProcessResult :: Error (FulfillmentErrorCode :: Subtype (expected_found , err)) } } } ty :: PredicateKind :: Coerce (coerce) => { match self . selcx . infcx . coerce_predicate (& obligation . cause , obligation . param_env , Binder :: dummy (coerce) ,) { Err ((a , b)) => { pending_obligation . stalled_on = vec ! [TyOrConstInferVar :: Ty (a) , TyOrConstInferVar :: Ty (b)] ; ProcessResult :: Unchanged } Ok (Ok (ok)) => { ProcessResult :: Changed (mk_pending (obligation , ok . obligations)) } Ok (Err (err)) => { let expected_found = ExpectedFound :: new (coerce . b , coerce . a) ; ProcessResult :: Error (FulfillmentErrorCode :: Subtype (expected_found , err)) } } } ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstEvaluatable (uv)) => { match const_evaluatable :: is_const_evaluatable (self . selcx . infcx , uv , obligation . param_env , obligation . cause . span ,) { Ok (()) => ProcessResult :: Changed (Default :: default ()) , Err (NotConstEvaluatable :: MentionsInfer) => { pending_obligation . stalled_on . clear () ; pending_obligation . stalled_on . extend (uv . walk () . filter_map (TyOrConstInferVar :: maybe_from_generic_arg) ,) ; ProcessResult :: Unchanged } Err (e @ NotConstEvaluatable :: MentionsParam | e @ NotConstEvaluatable :: Error (_) ,) => ProcessResult :: Error (FulfillmentErrorCode :: Select (SelectionError :: NotConstEvaluatable (e) ,)) , } } ty :: PredicateKind :: ConstEquate (c1 , c2) => { let tcx = self . selcx . tcx () ; assert ! (tcx . features () . generic_const_exprs () , "`ConstEquate` without a feature gate: {c1:?} {c2:?}" ,) ; { let c1 = tcx . expand_abstract_consts (c1) ; let c2 = tcx . expand_abstract_consts (c2) ; debug ! ("equating consts:\nc1= {:?}\nc2= {:?}" , c1 , c2) ; use rustc_hir :: def :: DefKind ; match (c1 . kind () , c2 . kind ()) { (ty :: ConstKind :: Unevaluated (a) , ty :: ConstKind :: Unevaluated (b)) if a . def == b . def && tcx . def_kind (a . def) == DefKind :: AssocConst => { if let Ok (new_obligations) = infcx . at (& obligation . cause , obligation . param_env) . eq (DefineOpaqueTypes :: Yes , ty :: AliasTerm :: from (a) , ty :: AliasTerm :: from (b) ,) { return ProcessResult :: Changed (mk_pending (obligation , new_obligations . into_obligations () ,)) ; } } (_ , ty :: ConstKind :: Unevaluated (_)) | (ty :: ConstKind :: Unevaluated (_) , _) => () , (_ , _) => { if let Ok (new_obligations) = infcx . at (& obligation . cause , obligation . param_env) . eq (DefineOpaqueTypes :: Yes , c1 , c2) { return ProcessResult :: Changed (mk_pending (obligation , new_obligations . into_obligations () ,)) ; } } } } let stalled_on = & mut pending_obligation . stalled_on ; let mut evaluate = | c : Const < 'tcx > | { if let ty :: ConstKind :: Unevaluated (unevaluated) = c . kind () { match super :: try_evaluate_const (self . selcx . infcx , c , obligation . param_env ,) { Ok (val) => Ok (val) , e @ Err (EvaluateConstErr :: HasGenericsOrInfers) => { stalled_on . extend (unevaluated . args . iter () . filter_map (TyOrConstInferVar :: maybe_from_generic_arg) ,) ; e } e @ Err (EvaluateConstErr :: EvaluationFailure (_) | EvaluateConstErr :: InvalidConstParamTy (_) ,) => e , } } else { Ok (c) } } ; match (evaluate (c1) , evaluate (c2)) { (Ok (c1) , Ok (c2)) => { match self . selcx . infcx . at (& obligation . cause , obligation . param_env) . eq (DefineOpaqueTypes :: Yes , c1 , c2 ,) { Ok (inf_ok) => ProcessResult :: Changed (mk_pending (obligation , inf_ok . into_obligations () ,)) , Err (err) => { ProcessResult :: Error (FulfillmentErrorCode :: ConstEquate (ExpectedFound :: new (c1 , c2) , err ,)) } } } (Err (EvaluateConstErr :: InvalidConstParamTy (e)) , _) | (_ , Err (EvaluateConstErr :: InvalidConstParamTy (e))) => { ProcessResult :: Error (FulfillmentErrorCode :: Select (SelectionError :: NotConstEvaluatable (NotConstEvaluatable :: Error (e)) ,)) } (Err (EvaluateConstErr :: EvaluationFailure (e)) , _) | (_ , Err (EvaluateConstErr :: EvaluationFailure (e))) => { ProcessResult :: Error (FulfillmentErrorCode :: Select (SelectionError :: NotConstEvaluatable (NotConstEvaluatable :: Error (e)) ,)) } (Err (EvaluateConstErr :: HasGenericsOrInfers) , _) | (_ , Err (EvaluateConstErr :: HasGenericsOrInfers)) => { if c1 . has_non_region_infer () || c2 . has_non_region_infer () { ProcessResult :: Unchanged } else { let expected_found = ExpectedFound :: new (c1 , c2) ; ProcessResult :: Error (FulfillmentErrorCode :: ConstEquate (expected_found , TypeError :: ConstMismatch (expected_found) ,)) } } } } ty :: PredicateKind :: Clause (ty :: ClauseKind :: UnstableFeature (symbol)) => { if may_use_unstable_feature (self . selcx . infcx , obligation . param_env , symbol) { ProcessResult :: Changed (Default :: default ()) } else { ProcessResult :: Unchanged } } } , } } #[inline (never)] fn process_backedge < 'c , I > (& mut self , cycle : I , _marker : PhantomData < & 'c PendingPredicateObligation < 'tcx > > ,) -> Result < () , FulfillmentErrorCode < 'tcx > > where I : Clone + Iterator < Item = & 'c PendingPredicateObligation < 'tcx > > , { if self . selcx . coinductive_match (cycle . clone () . map (| s | s . obligation . predicate)) { debug ! ("process_child_obligations: coinductive match") ; Ok (()) } else { let cycle = cycle . map (| c | c . obligation . clone ()) . collect () ; Err (FulfillmentErrorCode :: Cycle (cycle)) } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > FulfillProcessor < 'a , 'tcx > { #[instrument (level = "debug" , skip (self , obligation , stalled_on))] fn process_trait_obligation (& mut self , obligation : & PredicateObligation < 'tcx > , trait_obligation : PolyTraitObligation < 'tcx > , stalled_on : & mut Vec < TyOrConstInferVar > ,) -> ProcessResult < PendingPredicateObligation < 'tcx > , FulfillmentErrorCode < 'tcx > > { let infcx = self . selcx . infcx ; if obligation . predicate . is_global () && ! matches ! (infcx . typing_mode () , TypingMode :: Coherence) { if infcx . predicate_must_hold_considering_regions (obligation) { debug ! ("selecting trait at depth {} evaluated to holds" , obligation . recursion_depth) ; return ProcessResult :: Changed (Default :: default ()) ; } } match self . selcx . poly_select (& trait_obligation) { Ok (Some (impl_source)) => { debug ! ("selecting trait at depth {} yielded Ok(Some)" , obligation . recursion_depth) ; ProcessResult :: Changed (mk_pending (obligation , impl_source . nested_obligations ())) } Ok (None) => { debug ! ("selecting trait at depth {} yielded Ok(None)" , obligation . recursion_depth) ; stalled_on . clear () ; stalled_on . extend (args_infer_vars (& self . selcx , trait_obligation . predicate . map_bound (| pred | pred . trait_ref . args) ,)) ; debug ! ("process_predicate: pending obligation {:?} now stalled on {:?}" , infcx . resolve_vars_if_possible (obligation . clone ()) , stalled_on) ; ProcessResult :: Unchanged } Err (selection_err) => { debug ! ("selecting trait at depth {} yielded Err" , obligation . recursion_depth) ; ProcessResult :: Error (FulfillmentErrorCode :: Select (selection_err)) } } } fn process_projection_obligation (& mut self , obligation : & PredicateObligation < 'tcx > , project_obligation : PolyProjectionObligation < 'tcx > , stalled_on : & mut Vec < TyOrConstInferVar > ,) -> ProcessResult < PendingPredicateObligation < 'tcx > , FulfillmentErrorCode < 'tcx > > { let tcx = self . selcx . tcx () ; let infcx = self . selcx . infcx ; if obligation . predicate . is_global () && ! matches ! (infcx . typing_mode () , TypingMode :: Coherence) { if infcx . predicate_must_hold_considering_regions (obligation) { if let Some (key) = ProjectionCacheKey :: from_poly_projection_obligation (& mut self . selcx , & project_obligation ,) { infcx . inner . borrow_mut () . projection_cache () . complete (key , EvaluationResult :: EvaluatedToOk) ; } return ProcessResult :: Changed (Default :: default ()) ; } else { debug ! ("Does NOT hold: {:?}" , obligation) ; } } match project :: poly_project_and_unify_term (& mut self . selcx , & project_obligation) { ProjectAndUnifyResult :: Holds (os) => ProcessResult :: Changed (mk_pending (obligation , os)) , ProjectAndUnifyResult :: FailedNormalization => { stalled_on . clear () ; stalled_on . extend (args_infer_vars (& self . selcx , project_obligation . predicate . map_bound (| pred | pred . projection_term . args) ,)) ; ProcessResult :: Unchanged } ProjectAndUnifyResult :: Recursive => { let mut obligations = PredicateObligations :: with_capacity (1) ; obligations . push (project_obligation . with (tcx , project_obligation . predicate)) ; ProcessResult :: Changed (mk_pending (obligation , obligations)) } ProjectAndUnifyResult :: MismatchedProjectionTypes (e) => { ProcessResult :: Error (FulfillmentErrorCode :: Project (e)) } } } fn process_host_obligation (& mut self , obligation : & PredicateObligation < 'tcx > , host_obligation : HostEffectObligation < 'tcx > , stalled_on : & mut Vec < TyOrConstInferVar > ,) -> ProcessResult < PendingPredicateObligation < 'tcx > , FulfillmentErrorCode < 'tcx > > { match effects :: evaluate_host_effect_obligation (& mut self . selcx , & host_obligation) { Ok (nested) => ProcessResult :: Changed (mk_pending (obligation , nested)) , Err (effects :: EvaluationFailure :: Ambiguous) => { stalled_on . clear () ; stalled_on . extend (args_infer_vars (& self . selcx , ty :: Binder :: dummy (host_obligation . predicate . trait_ref . args) ,)) ; ProcessResult :: Unchanged } Err (effects :: EvaluationFailure :: NoSolution) => { ProcessResult :: Error (FulfillmentErrorCode :: Select (SelectionError :: Unimplemented)) } } } }}}

macro_rules! args_infer_vars_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function args_infer_vars in module {}", module_path!());
    };
}

mkfn!{
    args_infer_vars_introspect!();
    #[doc = " Returns the set of inference variables contained in `args`."] fn args_infer_vars < 'tcx > (selcx : & SelectionContext < '_ , 'tcx > , args : ty :: Binder < 'tcx , GenericArgsRef < 'tcx > > ,) -> impl Iterator < Item = TyOrConstInferVar > { selcx . infcx . resolve_vars_if_possible (args) . skip_binder () . iter () . filter (| arg | arg . has_non_region_infer ()) . flat_map (| arg | { let mut walker = arg . walk () ; while let Some (c) = walker . next () { if ! c . has_non_region_infer () { walker . visited . remove (& c) ; walker . skip_current_subtree () ; } } walker . visited . into_iter () }) . filter_map (TyOrConstInferVar :: maybe_from_generic_arg) }
}
mkitem!{mkstruct!{#[derive (Debug)] pub struct OldSolverError < 'tcx > (Error < PendingPredicateObligation < 'tcx > , FulfillmentErrorCode < 'tcx > > ,) ;}}
mkitem!{mkimpl!{impl < 'tcx > FromSolverError < 'tcx , OldSolverError < 'tcx > > for FulfillmentError < 'tcx > { fn from_solver_error (_infcx : & InferCtxt < 'tcx > , error : OldSolverError < 'tcx >) -> Self { let mut iter = error . 0 . backtrace . into_iter () ; let obligation = iter . next () . unwrap () . obligation ; let root_obligation = iter . next_back () . map_or_else (| | obligation . clone () , | e | e . obligation) ; FulfillmentError :: new (obligation , error . 0 . error , root_obligation) } }}}
mkitem!{mkimpl!{impl < 'tcx > FromSolverError < 'tcx , OldSolverError < 'tcx > > for ScrubbedTraitError < 'tcx > { fn from_solver_error (_infcx : & InferCtxt < 'tcx > , error : OldSolverError < 'tcx >) -> Self { match error . 0 . error { FulfillmentErrorCode :: Select (_) | FulfillmentErrorCode :: Project (_) | FulfillmentErrorCode :: Subtype (_ , _) | FulfillmentErrorCode :: ConstEquate (_ , _) => ScrubbedTraitError :: TrueError , FulfillmentErrorCode :: Ambiguity { overflow : _ } => ScrubbedTraitError :: Ambiguity , FulfillmentErrorCode :: Cycle (cycle) => ScrubbedTraitError :: Cycle (cycle) , } } }}}