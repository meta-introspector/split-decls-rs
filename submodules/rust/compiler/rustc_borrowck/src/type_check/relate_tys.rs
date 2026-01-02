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
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_infer :: infer :: relate :: { PredicateEmittingRelation , Relate , RelateResult , StructurallyRelateAliases , TypeRelation , } ;}
mkuse!{use rustc_infer :: infer :: { InferCtxt , NllRegionVariableOrigin } ;}
mkuse!{use rustc_infer :: traits :: Obligation ;}
mkuse!{use rustc_infer :: traits :: solve :: Goal ;}
mkuse!{use rustc_middle :: mir :: ConstraintCategory ;}
mkuse!{use rustc_middle :: traits :: ObligationCause ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: ty :: relate :: combine :: { super_combine_consts , super_combine_tys } ;}
mkuse!{use rustc_middle :: ty :: { self , FnMutDelegate , Ty , TyCtxt , TypeVisitableExt } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: constraints :: OutlivesConstraint ;}
mkuse!{use crate :: diagnostics :: UniverseInfo ;}
mkuse!{use crate :: renumber :: RegionCtxt ;}
mkuse!{use crate :: type_check :: { InstantiateOpaqueType , Locations , TypeChecker } ;}
mkitem!{mkimpl!{impl < 'a , 'tcx > TypeChecker < 'a , 'tcx > { #[doc = " Adds sufficient constraints to ensure that `a R b` where `R` depends on `v`:"] #[doc = ""] #[doc = " - \"Covariant\" `a <: b`"] #[doc = " - \"Invariant\" `a == b`"] #[doc = " - \"Contravariant\" `a :> b`"] #[doc = ""] #[doc = " N.B., the type `a` is permitted to have unresolved inference"] #[doc = " variables, but not the type `b`."] #[instrument (skip (self) , level = "debug")] pub (super) fn relate_types (& mut self , a : Ty < 'tcx > , v : ty :: Variance , b : Ty < 'tcx > , locations : Locations , category : ConstraintCategory < 'tcx > ,) -> Result < () , NoSolution > { NllTypeRelating :: new (self , locations , category , UniverseInfo :: relate (a , b) , v) . relate (a , b) ? ; Ok (()) } #[doc = " Add sufficient constraints to ensure `a == b`. See also [Self::relate_types]."] pub (super) fn eq_args (& mut self , a : ty :: GenericArgsRef < 'tcx > , b : ty :: GenericArgsRef < 'tcx > , locations : Locations , category : ConstraintCategory < 'tcx > ,) -> Result < () , NoSolution > { NllTypeRelating :: new (self , locations , category , UniverseInfo :: other () , ty :: Invariant) . relate (a , b) ? ; Ok (()) } }}}
mkitem!{mkstruct!{struct NllTypeRelating < 'a , 'b , 'tcx > { type_checker : & 'a mut TypeChecker < 'b , 'tcx > , #[doc = " Where (and why) is this relation taking place?"] locations : Locations , #[doc = " What category do we assign the resulting `'a: 'b` relationships?"] category : ConstraintCategory < 'tcx > , #[doc = " Information so that error reporting knows what types we are relating"] #[doc = " when reporting a bound region error."] universe_info : UniverseInfo < 'tcx > , #[doc = " How are we relating `a` and `b`?"] #[doc = ""] #[doc = " - Covariant means `a <: b`."] #[doc = " - Contravariant means `b <: a`."] #[doc = " - Invariant means `a == b`."] #[doc = " - Bivariant means that it doesn't matter."] ambient_variance : ty :: Variance , ambient_variance_info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'a , 'b , 'tcx > NllTypeRelating < 'a , 'b , 'tcx > { fn new (type_checker : & 'a mut TypeChecker < 'b , 'tcx > , locations : Locations , category : ConstraintCategory < 'tcx > , universe_info : UniverseInfo < 'tcx > , ambient_variance : ty :: Variance ,) -> Self { Self { type_checker , locations , category , universe_info , ambient_variance , ambient_variance_info : ty :: VarianceDiagInfo :: default () , } } fn ambient_covariance (& self) -> bool { match self . ambient_variance { ty :: Covariant | ty :: Invariant => true , ty :: Contravariant | ty :: Bivariant => false , } } fn ambient_contravariance (& self) -> bool { match self . ambient_variance { ty :: Contravariant | ty :: Invariant => true , ty :: Covariant | ty :: Bivariant => false , } } fn relate_opaques (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) -> RelateResult < 'tcx , () > { let infcx = self . type_checker . infcx ; debug_assert ! (! infcx . next_trait_solver ()) ; let mut enable_subtyping = | ty , opaque_is_expected | { let ty_vid = infcx . next_ty_vid (self . span ()) ; let variance = if opaque_is_expected { self . ambient_variance } else { self . ambient_variance . xform (ty :: Contravariant) } ; self . type_checker . infcx . instantiate_ty_var (self , opaque_is_expected , ty_vid , variance , ty ,) ? ; Ok (infcx . resolve_vars_if_possible (Ty :: new_infer (infcx . tcx , ty :: TyVar (ty_vid)))) } ; let (a , b) = match (a . kind () , b . kind ()) { (& ty :: Alias (ty :: Opaque , ..) , _) => (a , enable_subtyping (b , true) ?) , (_ , & ty :: Alias (ty :: Opaque , ..)) => (enable_subtyping (a , false) ? , b) , _ => unreachable ! ("expected at least one opaque type in `relate_opaques`, got {a} and {b}.") , } ; self . register_goals (infcx . handle_opaque_type (a , b , self . span () , self . param_env ()) ?) ; Ok (()) } fn enter_forall < T , U > (& mut self , binder : ty :: Binder < 'tcx , T > , f : impl FnOnce (& mut Self , T) -> U ,) -> U where T : ty :: TypeFoldable < TyCtxt < 'tcx > > + Copy , { let value = if let Some (inner) = binder . no_bound_vars () { inner } else { let infcx = self . type_checker . infcx ; let mut lazy_universe = None ; let delegate = FnMutDelegate { regions : & mut | br : ty :: BoundRegion | { let universe = lazy_universe . unwrap_or_else (| | { let universe = self . create_next_universe () ; lazy_universe = Some (universe) ; universe }) ; let placeholder = ty :: PlaceholderRegion { universe , bound : br } ; debug ! (? placeholder) ; let placeholder_reg = self . next_placeholder_region (placeholder) ; debug ! (? placeholder_reg) ; placeholder_reg } , types : & mut | _bound_ty : ty :: BoundTy | { unreachable ! ("we only replace regions in nll_relate, not types") } , consts : & mut | _bound_const : ty :: BoundConst | { unreachable ! ("we only replace regions in nll_relate, not consts") } , } ; infcx . tcx . replace_bound_vars_uncached (binder , delegate) } ; debug ! (? value) ; f (self , value) } #[instrument (skip (self) , level = "debug")] fn instantiate_binder_with_existentials < T > (& mut self , binder : ty :: Binder < 'tcx , T >) -> T where T : ty :: TypeFoldable < TyCtxt < 'tcx > > + Copy , { if let Some (inner) = binder . no_bound_vars () { return inner ; } let infcx = self . type_checker . infcx ; let mut reg_map = FxHashMap :: default () ; let delegate = FnMutDelegate { regions : & mut | br : ty :: BoundRegion | { if let Some (ex_reg_var) = reg_map . get (& br) { * ex_reg_var } else { let ex_reg_var = self . next_existential_region_var (br . kind . get_name (infcx . infcx . tcx)) ; debug ! (? ex_reg_var) ; reg_map . insert (br , ex_reg_var) ; ex_reg_var } } , types : & mut | _bound_ty : ty :: BoundTy | { unreachable ! ("we only replace regions in nll_relate, not types") } , consts : & mut | _bound_const : ty :: BoundConst | { unreachable ! ("we only replace regions in nll_relate, not consts") } , } ; let replaced = infcx . tcx . replace_bound_vars_uncached (binder , delegate) ; debug ! (? replaced) ; replaced } fn create_next_universe (& mut self) -> ty :: UniverseIndex { let universe = self . type_checker . infcx . create_next_universe () ; self . type_checker . constraints . universe_causes . insert (universe , self . universe_info . clone ()) ; universe } #[instrument (skip (self) , level = "debug")] fn next_existential_region_var (& mut self , name : Option < Symbol >) -> ty :: Region < 'tcx > { let origin = NllRegionVariableOrigin :: Existential { name } ; self . type_checker . infcx . next_nll_region_var (origin , | | RegionCtxt :: Existential (name)) } #[instrument (skip (self) , level = "debug")] fn next_placeholder_region (& mut self , placeholder : ty :: PlaceholderRegion) -> ty :: Region < 'tcx > { let reg = self . type_checker . constraints . placeholder_region (self . type_checker . infcx , placeholder) ; let reg_info = match placeholder . bound . kind { ty :: BoundRegionKind :: Anon => sym :: anon , ty :: BoundRegionKind :: Named (def_id) => self . type_checker . tcx () . item_name (def_id) , ty :: BoundRegionKind :: ClosureEnv => sym :: env , ty :: BoundRegionKind :: NamedAnon (_) => bug ! ("only used for pretty printing") , } ; if cfg ! (debug_assertions) { let mut var_to_origin = self . type_checker . infcx . reg_var_to_origin . borrow_mut () ; let new = RegionCtxt :: Placeholder (reg_info) ; let prev = var_to_origin . insert (reg . as_var () , new) ; if let Some (prev) = prev { assert_eq ! (new , prev) ; } } reg } fn push_outlives (& mut self , sup : ty :: Region < 'tcx > , sub : ty :: Region < 'tcx > , info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > ,) { let sub = self . type_checker . universal_regions . to_region_vid (sub) ; let sup = self . type_checker . universal_regions . to_region_vid (sup) ; self . type_checker . constraints . outlives_constraints . push (OutlivesConstraint { sup , sub , locations : self . locations , span : self . locations . span (self . type_checker . body) , category : self . category , variance_info : info , from_closure : false , }) ; } }}}
mkitem!{mkimpl!{impl < 'b , 'tcx > TypeRelation < TyCtxt < 'tcx > > for NllTypeRelating < '_ , 'b , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . type_checker . infcx . tcx } #[instrument (skip (self , info) , level = "trace" , ret)] fn relate_with_variance < T : Relate < TyCtxt < 'tcx > > > (& mut self , variance : ty :: Variance , info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , a : T , b : T ,) -> RelateResult < 'tcx , T > { let old_ambient_variance = self . ambient_variance ; self . ambient_variance = self . ambient_variance . xform (variance) ; self . ambient_variance_info = self . ambient_variance_info . xform (info) ; debug ! (? self . ambient_variance) ; let r = if self . ambient_variance == ty :: Bivariant { Ok (a) } else { self . relate (a , b) } ; self . ambient_variance = old_ambient_variance ; r } #[instrument (skip (self) , level = "debug")] fn tys (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) -> RelateResult < 'tcx , Ty < 'tcx > > { let infcx = self . type_checker . infcx ; let a = self . type_checker . infcx . shallow_resolve (a) ; assert ! (! b . has_non_region_infer () , "unexpected inference var {:?}" , b) ; if a == b { return Ok (a) ; } match (a . kind () , b . kind ()) { (_ , & ty :: Infer (ty :: TyVar (_))) => { span_bug ! (self . span () , "should not be relating type variables on the right in MIR typeck") ; } (& ty :: Infer (ty :: TyVar (a_vid)) , _) => { infcx . instantiate_ty_var (self , true , a_vid , self . ambient_variance , b) ? } (& ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id : a_def_id , .. }) , & ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id : b_def_id , .. }) ,) if a_def_id == b_def_id || infcx . next_trait_solver () => { super_combine_tys (& infcx . infcx , self , a , b) . map (| _ | ()) . or_else (| err | { assert ! (! self . type_checker . infcx . next_trait_solver ()) ; self . cx () . dcx () . span_delayed_bug (self . span () , "failure to relate an opaque to itself should result in an error later on" ,) ; if a_def_id . is_local () { self . relate_opaques (a , b) } else { Err (err) } }) ? ; } (& ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , .. }) , _) | (_ , & ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , .. })) if def_id . is_local () && ! self . type_checker . infcx . next_trait_solver () => { self . relate_opaques (a , b) ? ; } _ => { debug ! (? a , ? b , ? self . ambient_variance) ; super_combine_tys (& self . type_checker . infcx . infcx , self , a , b) ? ; } } Ok (a) } #[instrument (skip (self) , level = "trace")] fn regions (& mut self , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > ,) -> RelateResult < 'tcx , ty :: Region < 'tcx > > { debug ! (? self . ambient_variance) ; if self . ambient_covariance () { self . push_outlives (a , b , self . ambient_variance_info) ; } if self . ambient_contravariance () { self . push_outlives (b , a , self . ambient_variance_info) ; } Ok (a) } fn consts (& mut self , a : ty :: Const < 'tcx > , b : ty :: Const < 'tcx > ,) -> RelateResult < 'tcx , ty :: Const < 'tcx > > { let a = self . type_checker . infcx . shallow_resolve_const (a) ; assert ! (! a . has_non_region_infer () , "unexpected inference var {:?}" , a) ; assert ! (! b . has_non_region_infer () , "unexpected inference var {:?}" , b) ; super_combine_consts (& self . type_checker . infcx . infcx , self , a , b) } #[instrument (skip (self) , level = "trace")] fn binders < T > (& mut self , a : ty :: Binder < 'tcx , T > , b : ty :: Binder < 'tcx , T > ,) -> RelateResult < 'tcx , ty :: Binder < 'tcx , T > > where T : Relate < TyCtxt < 'tcx > > , { debug ! (? self . ambient_variance) ; if let (Some (a) , Some (b)) = (a . no_bound_vars () , b . no_bound_vars ()) { self . relate (a , b) ? ; return Ok (ty :: Binder :: dummy (a)) ; } match self . ambient_variance { ty :: Covariant => { self . enter_forall (b , | this , b | { let a = this . instantiate_binder_with_existentials (a) ; this . relate (a , b) }) ? ; } ty :: Contravariant => { self . enter_forall (a , | this , a | { let b = this . instantiate_binder_with_existentials (b) ; this . relate (a , b) }) ? ; } ty :: Invariant => { self . enter_forall (b , | this , b | { let a = this . instantiate_binder_with_existentials (a) ; this . relate (a , b) }) ? ; self . enter_forall (a , | this , a | { let b = this . instantiate_binder_with_existentials (b) ; this . relate (a , b) }) ? ; } ty :: Bivariant => { } } Ok (a) } }}}
mkitem!{mkimpl!{impl < 'b , 'tcx > PredicateEmittingRelation < InferCtxt < 'tcx > > for NllTypeRelating < '_ , 'b , 'tcx > { fn span (& self) -> Span { self . locations . span (self . type_checker . body) } fn structurally_relate_aliases (& self) -> StructurallyRelateAliases { StructurallyRelateAliases :: No } fn param_env (& self) -> ty :: ParamEnv < 'tcx > { self . type_checker . infcx . param_env } fn register_predicates (& mut self , obligations : impl IntoIterator < Item : ty :: Upcast < TyCtxt < 'tcx > , ty :: Predicate < 'tcx > > > ,) { let tcx = self . cx () ; let param_env = self . param_env () ; self . register_goals (obligations . into_iter () . map (| to_pred | Goal :: new (tcx , param_env , to_pred)) ,) ; } fn register_goals (& mut self , obligations : impl IntoIterator < Item = Goal < 'tcx , ty :: Predicate < 'tcx > > > ,) { let _ : Result < _ , ErrorGuaranteed > = self . type_checker . fully_perform_op (self . locations , self . category , InstantiateOpaqueType { obligations : obligations . into_iter () . map (| goal | { Obligation :: new (self . cx () , ObligationCause :: dummy_with_span (self . span ()) , goal . param_env , goal . predicate ,) }) . collect () , base_universe : None , region_constraints : None , } ,) ; } fn register_alias_relate_predicate (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) { self . register_predicates ([ty :: Binder :: dummy (match self . ambient_variance { ty :: Covariant => ty :: PredicateKind :: AliasRelate (a . into () , b . into () , ty :: AliasRelationDirection :: Subtype ,) , ty :: Contravariant => ty :: PredicateKind :: AliasRelate (b . into () , a . into () , ty :: AliasRelationDirection :: Subtype ,) , ty :: Invariant => ty :: PredicateKind :: AliasRelate (a . into () , b . into () , ty :: AliasRelationDirection :: Equate ,) , ty :: Bivariant => { unreachable ! ("cannot defer an alias-relate goal with Bivariant variance (yet?)") } })]) ; } }}}