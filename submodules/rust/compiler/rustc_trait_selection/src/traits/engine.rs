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
mkuse!{use std :: cell :: RefCell ;}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_infer :: infer :: at :: ToTrace ;}
mkuse!{use rustc_infer :: infer :: canonical :: { Canonical , CanonicalQueryResponse , CanonicalVarValues , QueryResponse , } ;}
mkuse!{use rustc_infer :: infer :: { DefineOpaqueTypes , InferCtxt , InferOk , RegionResolutionError , TypeTrace } ;}
mkuse!{use rustc_infer :: traits :: PredicateObligations ;}
mkuse!{use rustc_macros :: extension ;}
mkuse!{use rustc_middle :: arena :: ArenaAllocatable ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: ty :: error :: TypeError ;}
mkuse!{use rustc_middle :: ty :: relate :: Relate ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeFoldable , Upcast , Variance } ;}
mkuse!{use super :: { FromSolverError , FulfillmentContext , ScrubbedTraitError , TraitEngine } ;}
mkuse!{use crate :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use crate :: regions :: InferCtxtRegionExt ;}
mkuse!{use crate :: solve :: { FulfillmentCtxt as NextFulfillmentCtxt , NextSolverError } ;}
mkuse!{use crate :: traits :: fulfill :: OldSolverError ;}
mkuse!{use crate :: traits :: { FulfillmentError , NormalizeExt , Obligation , ObligationCause , PredicateObligation , StructurallyNormalizeExt , } ;}
mkitem!{mkimpl!{#[extension (pub trait TraitEngineExt <'tcx , E >)] impl < 'tcx , E > dyn TraitEngine < 'tcx , E > where E : FromSolverError < 'tcx , NextSolverError < 'tcx > > + FromSolverError < 'tcx , OldSolverError < 'tcx > > , { fn new (infcx : & InferCtxt < 'tcx >) -> Box < Self > { if infcx . next_trait_solver () { Box :: new (NextFulfillmentCtxt :: new (infcx)) } else { assert ! (! infcx . tcx . next_trait_solver_globally () , "using old solver even though new solver is enabled globally") ; Box :: new (FulfillmentContext :: new (infcx)) } } }}}
mkitem!{mkstruct!{#[doc = " Used if you want to have pleasant experience when dealing"] #[doc = " with obligations outside of hir or mir typeck."] pub struct ObligationCtxt < 'a , 'tcx , E = ScrubbedTraitError < 'tcx > > { pub infcx : & 'a InferCtxt < 'tcx > , engine : RefCell < Box < dyn TraitEngine < 'tcx , E > > > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ObligationCtxt < 'a , 'tcx , FulfillmentError < 'tcx > > { pub fn new_with_diagnostics (infcx : & 'a InferCtxt < 'tcx >) -> Self { Self { infcx , engine : RefCell :: new (< dyn TraitEngine < 'tcx , _ > > :: new (infcx)) } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ObligationCtxt < 'a , 'tcx , ScrubbedTraitError < 'tcx > > { pub fn new (infcx : & 'a InferCtxt < 'tcx >) -> Self { Self { infcx , engine : RefCell :: new (< dyn TraitEngine < 'tcx , _ > > :: new (infcx)) } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , E > ObligationCtxt < 'a , 'tcx , E > where E : 'tcx , { pub fn register_obligation (& self , obligation : PredicateObligation < 'tcx >) { self . engine . borrow_mut () . register_predicate_obligation (self . infcx , obligation) ; } pub fn register_obligations (& self , obligations : impl IntoIterator < Item = PredicateObligation < 'tcx > > ,) { for obligation in obligations { self . engine . borrow_mut () . register_predicate_obligation (self . infcx , obligation) } } pub fn register_infer_ok_obligations < T > (& self , infer_ok : InferOk < 'tcx , T >) -> T { let InferOk { value , obligations } = infer_ok ; self . engine . borrow_mut () . register_predicate_obligations (self . infcx , obligations) ; value } #[doc = " Requires that `ty` must implement the trait with `def_id` in"] #[doc = " the given environment. This trait must not have any type"] #[doc = " parameters (except for `Self`)."] pub fn register_bound (& self , cause : ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , ty : Ty < 'tcx > , def_id : DefId ,) { let tcx = self . infcx . tcx ; let trait_ref = ty :: TraitRef :: new (tcx , def_id , [ty]) ; self . register_obligation (Obligation { cause , recursion_depth : 0 , param_env , predicate : trait_ref . upcast (tcx) , }) ; } pub fn normalize < T : TypeFoldable < TyCtxt < 'tcx > > > (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , value : T ,) -> T { let infer_ok = self . infcx . at (cause , param_env) . normalize (value) ; self . register_infer_ok_obligations (infer_ok) } pub fn eq < T : ToTrace < 'tcx > > (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , expected : T , actual : T ,) -> Result < () , TypeError < 'tcx > > { self . infcx . at (cause , param_env) . eq (DefineOpaqueTypes :: Yes , expected , actual) . map (| infer_ok | self . register_infer_ok_obligations (infer_ok)) } pub fn eq_trace < T : Relate < TyCtxt < 'tcx > > > (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , trace : TypeTrace < 'tcx > , expected : T , actual : T ,) -> Result < () , TypeError < 'tcx > > { self . infcx . at (cause , param_env) . eq_trace (DefineOpaqueTypes :: Yes , trace , expected , actual) . map (| infer_ok | self . register_infer_ok_obligations (infer_ok)) } #[doc = " Checks whether `expected` is a subtype of `actual`: `expected <: actual`."] pub fn sub < T : ToTrace < 'tcx > > (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , expected : T , actual : T ,) -> Result < () , TypeError < 'tcx > > { self . infcx . at (cause , param_env) . sub (DefineOpaqueTypes :: Yes , expected , actual) . map (| infer_ok | self . register_infer_ok_obligations (infer_ok)) } pub fn relate < T : ToTrace < 'tcx > > (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , variance : Variance , expected : T , actual : T ,) -> Result < () , TypeError < 'tcx > > { self . infcx . at (cause , param_env) . relate (DefineOpaqueTypes :: Yes , expected , variance , actual) . map (| infer_ok | self . register_infer_ok_obligations (infer_ok)) } #[doc = " Checks whether `expected` is a supertype of `actual`: `expected :> actual`."] pub fn sup < T : ToTrace < 'tcx > > (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , expected : T , actual : T ,) -> Result < () , TypeError < 'tcx > > { self . infcx . at (cause , param_env) . sup (DefineOpaqueTypes :: Yes , expected , actual) . map (| infer_ok | self . register_infer_ok_obligations (infer_ok)) } #[doc = " Computes the least-upper-bound, or mutual supertype, of two values."] pub fn lub < T : ToTrace < 'tcx > > (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , expected : T , actual : T ,) -> Result < T , TypeError < 'tcx > > { self . infcx . at (cause , param_env) . lub (expected , actual) . map (| infer_ok | self . register_infer_ok_obligations (infer_ok)) } #[must_use] pub fn select_where_possible (& self) -> Vec < E > { self . engine . borrow_mut () . select_where_possible (self . infcx) } #[must_use] pub fn select_all_or_error (& self) -> Vec < E > { self . engine . borrow_mut () . select_all_or_error (self . infcx) } #[doc = " Returns the not-yet-processed and stalled obligations from the"] #[doc = " `ObligationCtxt`."] #[doc = ""] #[doc = " Takes ownership of the context as doing operations such as"] #[doc = " [`ObligationCtxt::eq`] afterwards will result in other obligations"] #[doc = " getting ignored. You can make a new `ObligationCtxt` if this"] #[doc = " needs to be done in a loop, for example."] #[must_use] pub fn into_pending_obligations (self) -> PredicateObligations < 'tcx > { self . engine . borrow () . pending_obligations () } #[doc = " Resolves regions and reports errors."] #[doc = ""] #[doc = " Takes ownership of the context as doing trait solving afterwards"] #[doc = " will result in region constraints getting ignored."] pub fn resolve_regions_and_report_errors (self , body_id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , assumed_wf_tys : impl IntoIterator < Item = Ty < 'tcx > > ,) -> Result < () , ErrorGuaranteed > { let errors = self . infcx . resolve_regions (body_id , param_env , assumed_wf_tys) ; if errors . is_empty () { Ok (()) } else { Err (self . infcx . err_ctxt () . report_region_errors (body_id , & errors)) } } #[doc = " Resolves regions and reports errors."] #[doc = ""] #[doc = " Takes ownership of the context as doing trait solving afterwards"] #[doc = " will result in region constraints getting ignored."] #[must_use] pub fn resolve_regions (self , body_id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , assumed_wf_tys : impl IntoIterator < Item = Ty < 'tcx > > ,) -> Vec < RegionResolutionError < 'tcx > > { self . infcx . resolve_regions (body_id , param_env , assumed_wf_tys) } }}}
mkitem!{mkimpl!{impl < 'tcx > ObligationCtxt < '_ , 'tcx , FulfillmentError < 'tcx > > { pub fn assumed_wf_types_and_report_errors (& self , param_env : ty :: ParamEnv < 'tcx > , def_id : LocalDefId ,) -> Result < FxIndexSet < Ty < 'tcx > > , ErrorGuaranteed > { self . assumed_wf_types (param_env , def_id) . map_err (| errors | self . infcx . err_ctxt () . report_fulfillment_errors (errors)) } }}}
mkitem!{mkimpl!{impl < 'tcx > ObligationCtxt < '_ , 'tcx , ScrubbedTraitError < 'tcx > > { pub fn make_canonicalized_query_response < T > (& self , inference_vars : CanonicalVarValues < 'tcx > , answer : T ,) -> Result < CanonicalQueryResponse < 'tcx , T > , NoSolution > where T : Debug + TypeFoldable < TyCtxt < 'tcx > > , Canonical < 'tcx , QueryResponse < 'tcx , T > > : ArenaAllocatable < 'tcx > , { self . infcx . make_canonicalized_query_response (inference_vars , answer , & mut * * self . engine . borrow_mut () ,) } }}}
mkitem!{mkimpl!{impl < 'tcx , E > ObligationCtxt < '_ , 'tcx , E > where E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { pub fn assumed_wf_types (& self , param_env : ty :: ParamEnv < 'tcx > , def_id : LocalDefId ,) -> Result < FxIndexSet < Ty < 'tcx > > , Vec < E > > { let tcx = self . infcx . tcx ; let mut implied_bounds = FxIndexSet :: default () ; let mut errors = Vec :: new () ; for & (ty , span) in tcx . assumed_wf_types (def_id) { let cause = ObligationCause :: misc (span , def_id) ; match self . infcx . at (& cause , param_env) . deeply_normalize (ty , & mut * * self . engine . borrow_mut ()) { Ok (normalized) => drop (implied_bounds . insert (normalized)) , Err (normalization_errors) => errors . extend (normalization_errors) , } ; } if errors . is_empty () { Ok (implied_bounds) } else { Err (errors) } } pub fn deeply_normalize < T : TypeFoldable < TyCtxt < 'tcx > > > (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , value : T ,) -> Result < T , Vec < E > > { self . infcx . at (cause , param_env) . deeply_normalize (value , & mut * * self . engine . borrow_mut ()) } pub fn structurally_normalize_ty (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , value : Ty < 'tcx > ,) -> Result < Ty < 'tcx > , Vec < E > > { self . infcx . at (cause , param_env) . structurally_normalize_ty (value , & mut * * self . engine . borrow_mut ()) } pub fn structurally_normalize_const (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , value : ty :: Const < 'tcx > ,) -> Result < ty :: Const < 'tcx > , Vec < E > > { self . infcx . at (cause , param_env) . structurally_normalize_const (value , & mut * * self . engine . borrow_mut ()) } pub fn structurally_normalize_term (& self , cause : & ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , value : ty :: Term < 'tcx > ,) -> Result < ty :: Term < 'tcx > , Vec < E > > { self . infcx . at (cause , param_env) . structurally_normalize_term (value , & mut * * self . engine . borrow_mut ()) } }}}