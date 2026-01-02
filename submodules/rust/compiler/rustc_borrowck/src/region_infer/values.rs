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
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: rc :: Rc ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexSet } ;}
mkuse!{use rustc_index :: Idx ;}
mkuse!{use rustc_index :: bit_set :: SparseBitMatrix ;}
mkuse!{use rustc_index :: interval :: { IntervalSet , SparseIntervalMatrix } ;}
mkuse!{use rustc_middle :: mir :: { BasicBlock , Location } ;}
mkuse!{use rustc_middle :: ty :: { self , RegionVid } ;}
mkuse!{use rustc_mir_dataflow :: points :: { DenseLocationMap , PointIndex } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: BorrowIndex ;}
mkuse!{use crate :: polonius :: LiveLoans ;}
mkitem!{rustc_index :: newtype_index ! { #[doc = " A single integer representing a `ty::Placeholder`."] #[debug_format = "PlaceholderIndex({})"] pub (crate) struct PlaceholderIndex { } }}
mkitem!{mkenum!{#[doc = " An individual element in a region value -- the value of a"] #[doc = " particular region variable consists of a set of these elements."] #[derive (Debug , Clone , PartialEq)] pub (crate) enum RegionElement { #[doc = " A point in the control-flow graph."] Location (Location) , #[doc = " A universally quantified region from the root universe (e.g.,"] #[doc = " a lifetime parameter)."] RootUniversalRegion (RegionVid) , #[doc = " A placeholder (e.g., instantiated from a `for<'a> fn(&'a u32)`"] #[doc = " type)."] PlaceholderRegion (ty :: PlaceholderRegion) , }}}
mkitem!{mkstruct!{#[doc = " Records the CFG locations where each region is live. When we initially compute liveness, we use"] #[doc = " an interval matrix storing liveness ranges for each region-vid."] #[derive (Clone)] pub (crate) struct LivenessValues { #[doc = " The map from locations to points."] location_map : Rc < DenseLocationMap > , #[doc = " Which regions are live. This is exclusive with the fine-grained tracking in `points`, and"] #[doc = " currently only used for validating promoteds (which don't care about more precise tracking)."] live_regions : Option < FxHashSet < RegionVid > > , #[doc = " For each region: the points where it is live."] #[doc = ""] #[doc = " This is not initialized for promoteds, because we don't care *where* within a promoted a"] #[doc = " region is live, only that it is."] points : Option < SparseIntervalMatrix < RegionVid , PointIndex > > , #[doc = " When using `-Zpolonius=next`, the set of loans that are live at a given point in the CFG."] live_loans : Option < LiveLoans > , }}}
mkitem!{mkimpl!{impl LivenessValues { #[doc = " Create an empty map of regions to locations where they're live."] pub (crate) fn with_specific_points (location_map : Rc < DenseLocationMap >) -> Self { LivenessValues { live_regions : None , points : Some (SparseIntervalMatrix :: new (location_map . num_points ())) , location_map , live_loans : None , } } #[doc = " Create an empty map of regions to locations where they're live."] #[doc = ""] #[doc = " Unlike `with_specific_points`, does not track exact locations where something is live, only"] #[doc = " which regions are live."] pub (crate) fn without_specific_points (location_map : Rc < DenseLocationMap >) -> Self { LivenessValues { live_regions : Some (Default :: default ()) , points : None , location_map , live_loans : None , } } #[doc = " Returns the liveness matrix of points where each region is live. Panics if the liveness"] #[doc = " values have been created without any per-point data (that is, for promoteds)."] pub (crate) fn points (& self) -> & SparseIntervalMatrix < RegionVid , PointIndex > { self . points . as_ref () . expect ("this `LivenessValues` wasn't created using `with_specific_points`") } #[doc = " Iterate through each region that has a value in this set."] pub (crate) fn regions (& self) -> impl Iterator < Item = RegionVid > { self . points . as_ref () . expect ("use with_specific_points") . rows () } #[doc = " Iterate through each region that has a value in this set."] #[rustc_lint_query_instability] #[allow (rustc :: potential_query_instability)] pub (crate) fn live_regions_unordered (& self) -> impl Iterator < Item = RegionVid > { self . live_regions . as_ref () . unwrap () . iter () . copied () } #[doc = " Records `region` as being live at the given `location`."] pub (crate) fn add_location (& mut self , region : RegionVid , location : Location) { let point = self . location_map . point_from_location (location) ; debug ! ("LivenessValues::add_location(region={:?}, location={:?})" , region , location) ; if let Some (points) = & mut self . points { points . insert (region , point) ; } else if self . location_map . point_in_range (point) { self . live_regions . as_mut () . unwrap () . insert (region) ; } } #[doc = " Records `region` as being live at all the given `points`."] pub (crate) fn add_points (& mut self , region : RegionVid , points : & IntervalSet < PointIndex >) { debug ! ("LivenessValues::add_points(region={:?}, points={:?})" , region , points) ; if let Some (this) = & mut self . points { this . union_row (region , points) ; } else if points . iter () . any (| point | self . location_map . point_in_range (point)) { self . live_regions . as_mut () . unwrap () . insert (region) ; } } #[doc = " Records `region` as being live at all the control-flow points."] pub (crate) fn add_all_points (& mut self , region : RegionVid) { if let Some (points) = & mut self . points { points . insert_all_into_row (region) ; } else { self . live_regions . as_mut () . unwrap () . insert (region) ; } } #[doc = " Returns whether `region` is marked live at the given `location`."] pub (crate) fn is_live_at (& self , region : RegionVid , location : Location) -> bool { let point = self . location_map . point_from_location (location) ; if let Some (points) = & self . points { points . row (region) . is_some_and (| r | r . contains (point)) } else { unreachable ! ("Should be using LivenessValues::with_specific_points to ask whether live at a location") } } #[doc = " Returns an iterator of all the points where `region` is live."] fn live_points (& self , region : RegionVid) -> impl Iterator < Item = PointIndex > { let Some (points) = & self . points else { unreachable ! ("Should be using LivenessValues::with_specific_points to ask whether live at a location") } ; points . row (region) . into_iter () . flat_map (| set | set . iter ()) . take_while (| & p | self . location_map . point_in_range (p)) } #[doc = " For debugging purposes, returns a pretty-printed string of the points where the `region` is"] #[doc = " live."] pub (crate) fn pretty_print_live_points (& self , region : RegionVid) -> String { pretty_print_region_elements (self . live_points (region) . map (| p | RegionElement :: Location (self . location_map . to_location (p))) ,) } #[inline] pub (crate) fn point_from_location (& self , location : Location) -> PointIndex { self . location_map . point_from_location (location) } #[inline] pub (crate) fn location_from_point (& self , point : PointIndex) -> Location { self . location_map . to_location (point) } #[doc = " When using `-Zpolonius=next`, records the given live loans for the loan scopes and active"] #[doc = " loans dataflow computations."] pub (crate) fn record_live_loans (& mut self , live_loans : LiveLoans) { self . live_loans = Some (live_loans) ; } #[doc = " When using `-Zpolonius=next`, returns whether the `loan_idx` is live at the given `point`."] pub (crate) fn is_loan_live_at (& self , loan_idx : BorrowIndex , point : PointIndex) -> bool { self . live_loans . as_ref () . expect ("Accessing live loans requires `-Zpolonius=next`") . contains (point , loan_idx) } }}}
mkitem!{mkstruct!{#[doc = " Maps from `ty::PlaceholderRegion` values that are used in the rest of"] #[doc = " rustc to the internal `PlaceholderIndex` values that are used in"] #[doc = " NLL."] #[derive (Debug , Default)] #[derive (Clone)] pub (crate) struct PlaceholderIndices { indices : FxIndexSet < ty :: PlaceholderRegion > , }}}
mkitem!{mkimpl!{impl PlaceholderIndices { #[doc = " Returns the `PlaceholderIndex` for the inserted `PlaceholderRegion`"] pub (crate) fn insert (& mut self , placeholder : ty :: PlaceholderRegion) -> PlaceholderIndex { let (index , _) = self . indices . insert_full (placeholder) ; index . into () } pub (crate) fn lookup_index (& self , placeholder : ty :: PlaceholderRegion) -> PlaceholderIndex { self . indices . get_index_of (& placeholder) . unwrap () . into () } pub (crate) fn lookup_placeholder (& self , placeholder : PlaceholderIndex ,) -> ty :: PlaceholderRegion { self . indices [placeholder . index ()] } pub (crate) fn len (& self) -> usize { self . indices . len () } }}}
mkitem!{mkstruct!{#[doc = " Stores the full values for a set of regions (in contrast to"] #[doc = " `LivenessValues`, which only stores those points in the where a"] #[doc = " region is live). The full value for a region may contain points in"] #[doc = " the CFG, but also free regions as well as bound universe"] #[doc = " placeholders."] #[doc = ""] #[doc = " Example:"] #[doc = ""] #[doc = " ```text"] #[doc = " fn foo(x: &'a u32) -> &'a u32 {"] #[doc = "    let y: &'0 u32 = x; // let's call this `'0`"] #[doc = "    y"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " Here, the variable `'0` would contain the free region `'a`,"] #[doc = " because (since it is returned) it must live for at least `'a`. But"] #[doc = " it would also contain various points from within the function."] pub (crate) struct RegionValues < N : Idx > { location_map : Rc < DenseLocationMap > , placeholder_indices : PlaceholderIndices , points : SparseIntervalMatrix < N , PointIndex > , free_regions : SparseBitMatrix < N , RegionVid > , #[doc = " Placeholders represent bound regions -- so something like `'a`"] #[doc = " in `for<'a> fn(&'a u32)`."] placeholders : SparseBitMatrix < N , PlaceholderIndex > , }}}
mkitem!{mkimpl!{impl < N : Idx > RegionValues < N > { #[doc = " Creates a new set of \"region values\" that tracks causal information."] #[doc = " Each of the regions in num_region_variables will be initialized with an"] #[doc = " empty set of points and no causal information."] pub (crate) fn new (location_map : Rc < DenseLocationMap > , num_universal_regions : usize , placeholder_indices : PlaceholderIndices ,) -> Self { let num_points = location_map . num_points () ; let num_placeholders = placeholder_indices . len () ; Self { location_map , points : SparseIntervalMatrix :: new (num_points) , placeholder_indices , free_regions : SparseBitMatrix :: new (num_universal_regions) , placeholders : SparseBitMatrix :: new (num_placeholders) , } } #[doc = " Adds the given element to the value for the given region. Returns whether"] #[doc = " the element is newly added (i.e., was not already present)."] pub (crate) fn add_element (& mut self , r : N , elem : impl ToElementIndex) -> bool { debug ! ("add(r={:?}, elem={:?})" , r , elem) ; elem . add_to_row (self , r) } #[doc = " Adds all the control-flow points to the values for `r`."] pub (crate) fn add_all_points (& mut self , r : N) { self . points . insert_all_into_row (r) ; } #[doc = " Adds all elements in `r_from` to `r_to` (because e.g., `r_to:"] #[doc = " r_from`)."] pub (crate) fn add_region (& mut self , r_to : N , r_from : N) -> bool { self . points . union_rows (r_from , r_to) | self . free_regions . union_rows (r_from , r_to) | self . placeholders . union_rows (r_from , r_to) } #[doc = " Returns `true` if the region `r` contains the given element."] pub (crate) fn contains (& self , r : N , elem : impl ToElementIndex) -> bool { elem . contained_in_row (self , r) } #[doc = " Returns the lowest statement index in `start..=end` which is not contained by `r`."] pub (crate) fn first_non_contained_inclusive (& self , r : N , block : BasicBlock , start : usize , end : usize ,) -> Option < usize > { let row = self . points . row (r) ? ; let block = self . location_map . entry_point (block) ; let start = block . plus (start) ; let end = block . plus (end) ; let first_unset = row . first_unset_in (start ..= end) ? ; Some (first_unset . index () - block . index ()) } #[doc = " `self[to] |= values[from]`, essentially: that is, take all the"] #[doc = " elements for the region `from` from `values` and add them to"] #[doc = " the region `to` in `self`."] pub (crate) fn merge_liveness (& mut self , to : N , from : RegionVid , values : & LivenessValues) { let Some (value_points) = & values . points else { panic ! ("LivenessValues must track specific points for use in merge_liveness") ; } ; if let Some (set) = value_points . row (from) { self . points . union_row (to , set) ; } } #[doc = " Returns `true` if `sup_region` contains all the CFG points that"] #[doc = " `sub_region` contains. Ignores universal regions."] pub (crate) fn contains_points (& self , sup_region : N , sub_region : N) -> bool { if let Some (sub_row) = self . points . row (sub_region) { if let Some (sup_row) = self . points . row (sup_region) { sup_row . superset (sub_row) } else { sub_row . is_empty () } } else { true } } #[doc = " Returns the locations contained within a given region `r`."] pub (crate) fn locations_outlived_by (& self , r : N) -> impl Iterator < Item = Location > { self . points . row (r) . into_iter () . flat_map (move | set | { set . iter () . take_while (move | & p | self . location_map . point_in_range (p)) . map (move | p | self . location_map . to_location (p)) }) } #[doc = " Returns just the universal regions that are contained in a given region's value."] pub (crate) fn universal_regions_outlived_by (& self , r : N) -> impl Iterator < Item = RegionVid > { self . free_regions . row (r) . into_iter () . flat_map (| set | set . iter ()) } #[doc = " Returns all the elements contained in a given region's value."] pub (crate) fn placeholders_contained_in (& self , r : N ,) -> impl Iterator < Item = ty :: PlaceholderRegion > { self . placeholders . row (r) . into_iter () . flat_map (| set | set . iter ()) . map (move | p | self . placeholder_indices . lookup_placeholder (p)) } #[doc = " Returns all the elements contained in a given region's value."] pub (crate) fn elements_contained_in (& self , r : N) -> impl Iterator < Item = RegionElement > { let points_iter = self . locations_outlived_by (r) . map (RegionElement :: Location) ; let free_regions_iter = self . universal_regions_outlived_by (r) . map (RegionElement :: RootUniversalRegion) ; let placeholder_universes_iter = self . placeholders_contained_in (r) . map (RegionElement :: PlaceholderRegion) ; points_iter . chain (free_regions_iter) . chain (placeholder_universes_iter) } #[doc = " Returns a \"pretty\" string value of the region. Meant for debugging."] pub (crate) fn region_value_str (& self , r : N) -> String { pretty_print_region_elements (self . elements_contained_in (r)) } }}}
mkitem!{mktrait!{pub (crate) trait ToElementIndex : Debug + Copy { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool ; fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool ; }}}
mkitem!{mkimpl!{impl ToElementIndex for Location { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool { let index = values . location_map . point_from_location (self) ; values . points . insert (row , index) } fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool { let index = values . location_map . point_from_location (self) ; values . points . contains (row , index) } }}}
mkitem!{mkimpl!{impl ToElementIndex for RegionVid { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool { values . free_regions . insert (row , self) } fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool { values . free_regions . contains (row , self) } }}}
mkitem!{mkimpl!{impl ToElementIndex for ty :: PlaceholderRegion { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool { let index = values . placeholder_indices . lookup_index (self) ; values . placeholders . insert (row , index) } fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool { let index = values . placeholder_indices . lookup_index (self) ; values . placeholders . contains (row , index) } }}}

macro_rules! pretty_print_points_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pretty_print_points in module {}", module_path!());
    };
}

mkfn!{
    pretty_print_points_introspect!();
    #[doc = " For debugging purposes, returns a pretty-printed string of the given points."] pub (crate) fn pretty_print_points (location_map : & DenseLocationMap , points : impl IntoIterator < Item = PointIndex > ,) -> String { pretty_print_region_elements (points . into_iter () . take_while (| & p | location_map . point_in_range (p)) . map (| p | location_map . to_location (p)) . map (RegionElement :: Location) ,) }
}

macro_rules! pretty_print_region_elements_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pretty_print_region_elements in module {}", module_path!());
    };
}

mkfn!{
    pretty_print_region_elements_introspect!();
    #[doc = " For debugging purposes, returns a pretty-printed string of the given region elements."] fn pretty_print_region_elements (elements : impl IntoIterator < Item = RegionElement >) -> String { let mut result = String :: new () ; result . push ('{') ; let mut open_location : Option < (Location , Location) > = None ; let mut sep = "" ; let mut push_sep = | s : & mut String | { s . push_str (sep) ; sep = ", " ; } ; for element in elements { match element { RegionElement :: Location (l) => { if let Some ((location1 , location2)) = open_location { if location2 . block == l . block && location2 . statement_index == l . statement_index - 1 { open_location = Some ((location1 , l)) ; continue ; } push_sep (& mut result) ; push_location_range (& mut result , location1 , location2) ; } open_location = Some ((l , l)) ; } RegionElement :: RootUniversalRegion (fr) => { if let Some ((location1 , location2)) = open_location { push_sep (& mut result) ; push_location_range (& mut result , location1 , location2) ; open_location = None ; } push_sep (& mut result) ; result . push_str (& format ! ("{fr:?}")) ; } RegionElement :: PlaceholderRegion (placeholder) => { if let Some ((location1 , location2)) = open_location { push_sep (& mut result) ; push_location_range (& mut result , location1 , location2) ; open_location = None ; } push_sep (& mut result) ; result . push_str (& format ! ("{placeholder:?}")) ; } } } if let Some ((location1 , location2)) = open_location { push_sep (& mut result) ; push_location_range (& mut result , location1 , location2) ; } result . push ('}') ; return result ; fn push_location_range (s : & mut String , location1 : Location , location2 : Location) { if location1 == location2 { s . push_str (& format ! ("{location1:?}")) ; } else { assert_eq ! (location1 . block , location2 . block) ; s . push_str (& format ! ("{:?}[{}..={}]" , location1 . block , location1 . statement_index , location2 . statement_index)) ; } } }
}