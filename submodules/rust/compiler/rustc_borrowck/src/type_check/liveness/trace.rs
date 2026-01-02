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
mkuse!{use rustc_data_structures :: fx :: { FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_index :: interval :: IntervalSet ;}
mkuse!{use rustc_infer :: infer :: canonical :: QueryRegionConstraints ;}
mkuse!{use rustc_infer :: infer :: outlives :: for_liveness ;}
mkuse!{use rustc_middle :: mir :: { BasicBlock , Body , ConstraintCategory , HasLocalDecls , Local , Location } ;}
mkuse!{use rustc_middle :: traits :: query :: DropckOutlivesResult ;}
mkuse!{use rustc_middle :: ty :: relate :: Relate ;}
mkuse!{use rustc_middle :: ty :: { Ty , TyCtxt , TypeVisitable , TypeVisitableExt } ;}
mkuse!{use rustc_mir_dataflow :: impls :: MaybeInitializedPlaces ;}
mkuse!{use rustc_mir_dataflow :: move_paths :: { HasMoveData , MoveData , MovePathIndex } ;}
mkuse!{use rustc_mir_dataflow :: points :: { DenseLocationMap , PointIndex } ;}
mkuse!{use rustc_mir_dataflow :: { Analysis , ResultsCursor } ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed , Span } ;}
mkuse!{use rustc_trait_selection :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use rustc_trait_selection :: traits :: ObligationCtxt ;}
mkuse!{use rustc_trait_selection :: traits :: query :: dropck_outlives ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: { DropckOutlives , TypeOp , TypeOpOutput } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: polonius ;}
mkuse!{use crate :: region_infer :: values ;}
mkuse!{use crate :: type_check :: liveness :: local_use_map :: LocalUseMap ;}
mkuse!{use crate :: type_check :: { NormalizeLocation , TypeChecker } ;}

macro_rules! trace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace in module {}", module_path!());
    };
}

mkfn!{
    trace_introspect!();
    #[doc = " This is the heart of the liveness computation. For each variable X"] #[doc = " that requires a liveness computation, it walks over all the uses"] #[doc = " of X and does a reverse depth-first search (\"trace\") through the"] #[doc = " MIR. This search stops when we find a definition of that variable."] #[doc = " The points visited in this search is the USE-LIVE set for the variable;"] #[doc = " of those points is added to all the regions that appear in the variable's"] #[doc = " type."] #[doc = ""] #[doc = " We then also walks through each *drop* of those variables and does"] #[doc = " another search, stopping when we reach a use or definition. This"] #[doc = " is the DROP-LIVE set of points. Each of the points in the"] #[doc = " DROP-LIVE set are to the liveness sets for regions found in the"] #[doc = " `dropck_outlives` result of the variable's type (in particular,"] #[doc = " this respects `#[may_dangle]` annotations)."] pub (super) fn trace < 'tcx > (typeck : & mut TypeChecker < '_ , 'tcx > , location_map : & DenseLocationMap , move_data : & MoveData < 'tcx > , relevant_live_locals : Vec < Local > , boring_locals : Vec < Local > ,) { let local_use_map = & LocalUseMap :: build (& relevant_live_locals , location_map , typeck . body) ; let cx = LivenessContext { typeck , flow_inits : None , location_map , local_use_map , move_data , drop_data : FxIndexMap :: default () , } ; let mut results = LivenessResults :: new (cx) ; results . add_extra_drop_facts (& relevant_live_locals) ; results . compute_for_all_locals (relevant_live_locals) ; results . dropck_boring_locals (boring_locals) ; }
}
mkitem!{mkstruct!{#[doc = " Contextual state for the type-liveness coroutine."] struct LivenessContext < 'a , 'typeck , 'tcx > { #[doc = " Current type-checker, giving us our inference context etc."] #[doc = ""] #[doc = " This also stores the body we're currently analyzing."] typeck : & 'a mut TypeChecker < 'typeck , 'tcx > , #[doc = " Defines the `PointIndex` mapping"] location_map : & 'a DenseLocationMap , #[doc = " Mapping to/from the various indices used for initialization tracking."] move_data : & 'a MoveData < 'tcx > , #[doc = " Cache for the results of `dropck_outlives` query."] drop_data : FxIndexMap < Ty < 'tcx > , DropData < 'tcx > > , #[doc = " Results of dataflow tracking which variables (and paths) have been"] #[doc = " initialized. Computed lazily when needed by drop-liveness."] flow_inits : Option < ResultsCursor < 'a , 'tcx , MaybeInitializedPlaces < 'a , 'tcx > > > , #[doc = " Index indicating where each variable is assigned, used, or"] #[doc = " dropped."] local_use_map : & 'a LocalUseMap , }}}
mkitem!{mkstruct!{struct DropData < 'tcx > { dropck_result : DropckOutlivesResult < 'tcx > , region_constraint_data : Option < & 'tcx QueryRegionConstraints < 'tcx > > , }}}
mkitem!{mkstruct!{struct LivenessResults < 'a , 'typeck , 'tcx > { cx : LivenessContext < 'a , 'typeck , 'tcx > , #[doc = " Set of points that define the current local."] defs : DenseBitSet < PointIndex > , #[doc = " Points where the current variable is \"use live\" -- meaning"] #[doc = " that there is a future \"full use\" that may use its value."] use_live_at : IntervalSet < PointIndex > , #[doc = " Points where the current variable is \"drop live\" -- meaning"] #[doc = " that there is no future \"full use\" that may use its value, but"] #[doc = " there is a future drop."] drop_live_at : IntervalSet < PointIndex > , #[doc = " Locations where drops may occur."] drop_locations : Vec < Location > , #[doc = " Stack used when doing (reverse) DFS."] stack : Vec < PointIndex > , }}}
mkitem!{mkimpl!{impl < 'a , 'typeck , 'tcx > LivenessResults < 'a , 'typeck , 'tcx > { fn new (cx : LivenessContext < 'a , 'typeck , 'tcx >) -> Self { let num_points = cx . location_map . num_points () ; LivenessResults { cx , defs : DenseBitSet :: new_empty (num_points) , use_live_at : IntervalSet :: new (num_points) , drop_live_at : IntervalSet :: new (num_points) , drop_locations : vec ! [] , stack : vec ! [] , } } fn compute_for_all_locals (& mut self , relevant_live_locals : Vec < Local >) { for local in relevant_live_locals { self . reset_local_state () ; self . add_defs_for (local) ; self . compute_use_live_points_for (local) ; self . compute_drop_live_points_for (local) ; let local_ty = self . cx . body () . local_decls [local] . ty ; if ! self . use_live_at . is_empty () { self . cx . add_use_live_facts_for (local_ty , & self . use_live_at) ; } if ! self . drop_live_at . is_empty () { self . cx . add_drop_live_facts_for (local , local_ty , & self . drop_locations , & self . drop_live_at ,) ; } } } #[doc = " Runs dropck for locals whose liveness isn't relevant. This is"] #[doc = " necessary to eagerly detect unbound recursion during drop glue computation."] #[doc = ""] #[doc = " These are all the locals which do not potentially reference a region local"] #[doc = " to this body. Locals which only reference free regions are always drop-live"] #[doc = " and can therefore safely be dropped."] fn dropck_boring_locals (& mut self , boring_locals : Vec < Local >) { for local in boring_locals { let local_ty = self . cx . body () . local_decls [local] . ty ; let local_span = self . cx . body () . local_decls [local] . source_info . span ; let drop_data = self . cx . drop_data . entry (local_ty) . or_insert_with ({ let typeck = & self . cx . typeck ; move | | LivenessContext :: compute_drop_data (typeck , local_ty , local_span) }) ; drop_data . dropck_result . report_overflows (self . cx . typeck . infcx . tcx , self . cx . typeck . body . local_decls [local] . source_info . span , local_ty ,) ; } } #[doc = " Add extra drop facts needed for Polonius."] #[doc = ""] #[doc = " Add facts for all locals with free regions, since regions may outlive"] #[doc = " the function body only at certain nodes in the CFG."] fn add_extra_drop_facts (& mut self , relevant_live_locals : & [Local]) { let Some (facts) = self . cx . typeck . polonius_facts . as_ref () else { return } ; let facts_to_add : Vec < _ > = { let relevant_live_locals : FxIndexSet < _ > = relevant_live_locals . iter () . copied () . collect () ; facts . var_dropped_at . iter () . filter_map (| & (local , location_index) | { let local_ty = self . cx . body () . local_decls [local] . ty ; if relevant_live_locals . contains (& local) || ! local_ty . has_free_regions () { return None ; } let location = self . cx . typeck . location_table . to_location (location_index) ; Some ((local , local_ty , location)) }) . collect () } ; let live_at = IntervalSet :: new (self . cx . location_map . num_points ()) ; for (local , local_ty , location) in facts_to_add { self . cx . add_drop_live_facts_for (local , local_ty , & [location] , & live_at) ; } } #[doc = " Clear the value of fields that are \"per local variable\"."] fn reset_local_state (& mut self) { self . defs . clear () ; self . use_live_at . clear () ; self . drop_live_at . clear () ; self . drop_locations . clear () ; assert ! (self . stack . is_empty ()) ; } #[doc = " Adds the definitions of `local` into `self.defs`."] fn add_defs_for (& mut self , local : Local) { for def in self . cx . local_use_map . defs (local) { debug ! ("- defined at {:?}" , def) ; self . defs . insert (def) ; } } #[doc = " Computes all points where local is \"use live\" -- meaning its"] #[doc = " current value may be used later (except by a drop). This is"] #[doc = " done by walking backwards from each use of `local` until we"] #[doc = " find a `def` of local."] #[doc = ""] #[doc = " Requires `add_defs_for(local)` to have been executed."] fn compute_use_live_points_for (& mut self , local : Local) { debug ! ("compute_use_live_points_for(local={:?})" , local) ; self . stack . extend (self . cx . local_use_map . uses (local)) ; while let Some (p) = self . stack . pop () { let block_start = self . cx . location_map . to_block_start (p) ; let previous_defs = self . defs . last_set_in (block_start ..= p) ; let previous_live_at = self . use_live_at . last_set_in (block_start ..= p) ; let exclusive_start = match (previous_defs , previous_live_at) { (Some (a) , Some (b)) => Some (std :: cmp :: max (a , b)) , (Some (a) , None) | (None , Some (a)) => Some (a) , (None , None) => None , } ; if let Some (exclusive) = exclusive_start { self . use_live_at . insert_range (exclusive + 1 ..= p) ; continue ; } else { self . use_live_at . insert_range (block_start ..= p) ; let block = self . cx . location_map . to_location (block_start) . block ; self . stack . extend (self . cx . body () . basic_blocks . predecessors () [block] . iter () . map (| & pred_bb | self . cx . body () . terminator_loc (pred_bb)) . map (| pred_loc | self . cx . location_map . point_from_location (pred_loc)) ,) ; } } } #[doc = " Computes all points where local is \"drop live\" -- meaning its"] #[doc = " current value may be dropped later (but not used). This is"] #[doc = " done by iterating over the drops of `local` where `local` (or"] #[doc = " some subpart of `local`) is initialized. For each such drop,"] #[doc = " we walk backwards until we find a point where `local` is"] #[doc = " either defined or use-live."] #[doc = ""] #[doc = " Requires `compute_use_live_points_for` and `add_defs_for` to"] #[doc = " have been executed."] fn compute_drop_live_points_for (& mut self , local : Local) { debug ! ("compute_drop_live_points_for(local={:?})" , local) ; let Some (mpi) = self . cx . move_data . rev_lookup . find_local (local) else { return } ; debug ! ("compute_drop_live_points_for: mpi = {:?}" , mpi) ; for drop_point in self . cx . local_use_map . drops (local) { let location = self . cx . location_map . to_location (drop_point) ; debug_assert_eq ! (self . cx . body () . terminator_loc (location . block) , location ,) ; if self . cx . initialized_at_terminator (location . block , mpi) && self . drop_live_at . insert (drop_point) { self . drop_locations . push (location) ; self . stack . push (drop_point) ; } } debug ! ("compute_drop_live_points_for: drop_locations={:?}" , self . drop_locations) ; while let Some (term_point) = self . stack . pop () { self . compute_drop_live_points_for_block (mpi , term_point) ; } } #[doc = " Executes one iteration of the drop-live analysis loop."] #[doc = ""] #[doc = " The parameter `mpi` is the `MovePathIndex` of the local variable"] #[doc = " we are currently analyzing."] #[doc = ""] #[doc = " The point `term_point` represents some terminator in the MIR,"] #[doc = " where the local `mpi` is drop-live on entry to that terminator."] #[doc = ""] #[doc = " This method adds all drop-live points within the block and --"] #[doc = " where applicable -- pushes the terminators of preceding blocks"] #[doc = " onto `self.stack`."] fn compute_drop_live_points_for_block (& mut self , mpi : MovePathIndex , term_point : PointIndex) { debug ! ("compute_drop_live_points_for_block(mpi={:?}, term_point={:?})" , self . cx . move_data . move_paths [mpi] . place , self . cx . location_map . to_location (term_point) ,) ; debug_assert ! (self . drop_live_at . contains (term_point)) ; let term_location = self . cx . location_map . to_location (term_point) ; debug_assert_eq ! (self . cx . body () . terminator_loc (term_location . block) , term_location ,) ; let block = term_location . block ; let entry_point = self . cx . location_map . entry_point (term_location . block) ; for p in (entry_point .. term_point) . rev () { debug ! ("compute_drop_live_points_for_block: p = {:?}" , self . cx . location_map . to_location (p)) ; if self . defs . contains (p) { debug ! ("compute_drop_live_points_for_block: def site") ; return ; } if self . use_live_at . contains (p) { debug ! ("compute_drop_live_points_for_block: use-live at {:?}" , p) ; return ; } if ! self . drop_live_at . insert (p) { debug ! ("compute_drop_live_points_for_block: already drop-live") ; return ; } } let body = self . cx . typeck . body ; for & pred_block in body . basic_blocks . predecessors () [block] . iter () { debug ! ("compute_drop_live_points_for_block: pred_block = {:?}" , pred_block ,) ; if ! self . cx . initialized_at_exit (pred_block , mpi) { debug ! ("compute_drop_live_points_for_block: not initialized") ; continue ; } let pred_term_loc = self . cx . body () . terminator_loc (pred_block) ; let pred_term_point = self . cx . location_map . point_from_location (pred_term_loc) ; if self . defs . contains (pred_term_point) { debug ! ("compute_drop_live_points_for_block: defined at {:?}" , pred_term_loc) ; continue ; } if self . use_live_at . contains (pred_term_point) { debug ! ("compute_drop_live_points_for_block: use-live at {:?}" , pred_term_loc) ; continue ; } if self . drop_live_at . insert (pred_term_point) { debug ! ("compute_drop_live_points_for_block: pushed to stack") ; self . stack . push (pred_term_point) ; } } } }}}
mkitem!{mkimpl!{impl < 'a , 'typeck , 'tcx > LivenessContext < 'a , 'typeck , 'tcx > { #[doc = " Computes the `MaybeInitializedPlaces` dataflow analysis if it hasn't been done already."] #[doc = ""] #[doc = " In practice, the results of this dataflow analysis are rarely needed but can be expensive to"] #[doc = " compute on big functions, so we compute them lazily as a fast path when:"] #[doc = " - there are relevant live locals"] #[doc = " - there are drop points for these relevant live locals."] #[doc = ""] #[doc = " This happens as part of the drop-liveness computation: it's the only place checking for"] #[doc = " maybe-initializedness of `MovePathIndex`es."] fn flow_inits (& mut self) -> & mut ResultsCursor < 'a , 'tcx , MaybeInitializedPlaces < 'a , 'tcx > > { self . flow_inits . get_or_insert_with (| | { let tcx = self . typeck . tcx () ; let body = self . typeck . body ; let flow_inits = MaybeInitializedPlaces :: new (tcx , body , self . move_data) . iterate_to_fixpoint (tcx , body , Some ("borrowck")) . into_results_cursor (body) ; flow_inits }) } }}}
mkitem!{mkimpl!{impl < 'tcx > LivenessContext < '_ , '_ , 'tcx > { fn body (& self) -> & Body < 'tcx > { self . typeck . body } #[doc = " Returns `true` if the local variable (or some part of it) is initialized at the current"] #[doc = " cursor position. Callers should call one of the `seek` methods immediately before to point"] #[doc = " the cursor to the desired location."] fn initialized_at_curr_loc (& mut self , mpi : MovePathIndex) -> bool { let flow_inits = self . flow_inits () ; let state = flow_inits . get () ; if state . contains (mpi) { return true ; } let move_paths = & flow_inits . analysis () . move_data () . move_paths ; move_paths [mpi] . find_descendant (move_paths , | mpi | state . contains (mpi)) . is_some () } #[doc = " Returns `true` if the local variable (or some part of it) is initialized in"] #[doc = " the terminator of `block`. We need to check this to determine if a"] #[doc = " DROP of some local variable will have an effect -- note that"] #[doc = " drops, as they may unwind, are always terminators."] fn initialized_at_terminator (& mut self , block : BasicBlock , mpi : MovePathIndex) -> bool { let terminator_location = self . body () . terminator_loc (block) ; self . flow_inits () . seek_before_primary_effect (terminator_location) ; self . initialized_at_curr_loc (mpi) } #[doc = " Returns `true` if the path `mpi` (or some part of it) is initialized at"] #[doc = " the exit of `block`."] #[doc = ""] #[doc = " **Warning:** Does not account for the result of `Call`"] #[doc = " instructions."] fn initialized_at_exit (& mut self , block : BasicBlock , mpi : MovePathIndex) -> bool { let terminator_location = self . body () . terminator_loc (block) ; self . flow_inits () . seek_after_primary_effect (terminator_location) ; self . initialized_at_curr_loc (mpi) } #[doc = " Stores the result that all regions in `value` are live for the"] #[doc = " points `live_at`."] fn add_use_live_facts_for (& mut self , value : Ty < 'tcx > , live_at : & IntervalSet < PointIndex >) { debug ! ("add_use_live_facts_for(value={:?})" , value) ; Self :: make_all_regions_live (self . location_map , self . typeck , value , live_at) ; } #[doc = " Some variable with type `live_ty` is \"drop live\" at `location`"] #[doc = " -- i.e., it may be dropped later. This means that *some* of"] #[doc = " the regions in its type must be live at `location`. The"] #[doc = " precise set will depend on the dropck constraints, and in"] #[doc = " particular this takes `#[may_dangle]` into account."] fn add_drop_live_facts_for (& mut self , dropped_local : Local , dropped_ty : Ty < 'tcx > , drop_locations : & [Location] , live_at : & IntervalSet < PointIndex > ,) { debug ! ("add_drop_live_constraint(\
             dropped_local={:?}, \
             dropped_ty={:?}, \
             drop_locations={:?}, \
             live_at={:?})" , dropped_local , dropped_ty , drop_locations , values :: pretty_print_points (self . location_map , live_at . iter ()) ,) ; let local_span = self . body () . local_decls () [dropped_local] . source_info . span ; let drop_data = self . drop_data . entry (dropped_ty) . or_insert_with ({ let typeck = & self . typeck ; move | | Self :: compute_drop_data (typeck , dropped_ty , local_span) }) ; if let Some (data) = & drop_data . region_constraint_data { for & drop_location in drop_locations { self . typeck . push_region_constraints (drop_location . to_locations () , ConstraintCategory :: Boring , data ,) ; } } drop_data . dropck_result . report_overflows (self . typeck . infcx . tcx , self . typeck . body . source_info (* drop_locations . first () . unwrap ()) . span , dropped_ty ,) ; for & kind in & drop_data . dropck_result . kinds { Self :: make_all_regions_live (self . location_map , self . typeck , kind , live_at) ; polonius :: legacy :: emit_drop_facts (self . typeck . tcx () , dropped_local , & kind , self . typeck . universal_regions , self . typeck . polonius_facts ,) ; } } fn make_all_regions_live (location_map : & DenseLocationMap , typeck : & mut TypeChecker < '_ , 'tcx > , value : impl TypeVisitable < TyCtxt < 'tcx > > + Relate < TyCtxt < 'tcx > > , live_at : & IntervalSet < PointIndex > ,) { debug ! ("make_all_regions_live(value={:?})" , value) ; debug ! ("make_all_regions_live: live_at={}" , values :: pretty_print_points (location_map , live_at . iter ()) ,) ; value . visit_with (& mut for_liveness :: FreeRegionsVisitor { tcx : typeck . tcx () , param_env : typeck . infcx . param_env , op : | r | { let live_region_vid = typeck . universal_regions . to_region_vid (r) ; typeck . constraints . liveness_constraints . add_points (live_region_vid , live_at) ; } , }) ; if let Some (polonius_liveness) = typeck . polonius_liveness . as_mut () { polonius_liveness . record_live_region_variance (typeck . infcx . tcx , typeck . universal_regions , value ,) ; } } fn compute_drop_data (typeck : & TypeChecker < '_ , 'tcx > , dropped_ty : Ty < 'tcx > , span : Span ,) -> DropData < 'tcx > { debug ! ("compute_drop_data(dropped_ty={:?})" , dropped_ty) ; let op = typeck . infcx . param_env . and (DropckOutlives { dropped_ty }) ; match op . fully_perform (typeck . infcx , typeck . root_cx . root_def_id () , DUMMY_SP) { Ok (TypeOpOutput { output , constraints , .. }) => { DropData { dropck_result : output , region_constraint_data : constraints } } Err (ErrorGuaranteed { .. }) => { typeck . infcx . probe (| _ | { let ocx = ObligationCtxt :: new_with_diagnostics (& typeck . infcx) ; let errors = match dropck_outlives :: compute_dropck_outlives_with_errors (& ocx , op , span ,) { Ok (_) => ocx . select_all_or_error () , Err (e) => e , } ; if ! errors . is_empty () { typeck . infcx . err_ctxt () . report_fulfillment_errors (errors) ; } }) ; DropData { dropck_result : Default :: default () , region_constraint_data : None } } } } }}}