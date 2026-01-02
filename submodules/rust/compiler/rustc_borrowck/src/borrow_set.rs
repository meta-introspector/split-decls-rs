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
mkuse!{use std :: fmt ;}
mkuse!{use std :: ops :: Index ;}
mkuse!{use rustc_data_structures :: fx :: { FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_middle :: mir :: visit :: { MutatingUseContext , NonUseContext , PlaceContext , Visitor } ;}
mkuse!{use rustc_middle :: mir :: { self , Body , Local , Location , traversal } ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: { RegionVid , TyCtxt } ;}
mkuse!{use rustc_mir_dataflow :: move_paths :: MoveData ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: BorrowIndex ;}
mkuse!{use crate :: place_ext :: PlaceExt ;}
mkitem!{mkstruct!{pub struct BorrowSet < 'tcx > { #[doc = " The fundamental map relating bitvector indexes to the borrows"] #[doc = " in the MIR. Each borrow is also uniquely identified in the MIR"] #[doc = " by the `Location` of the assignment statement in which it"] #[doc = " appears on the right hand side. Thus the location is the map"] #[doc = " key, and its position in the map corresponds to `BorrowIndex`."] pub (crate) location_map : FxIndexMap < Location , BorrowData < 'tcx > > , #[doc = " Locations which activate borrows."] #[doc = " NOTE: a given location may activate more than one borrow in the future"] #[doc = " when more general two-phase borrow support is introduced, but for now we"] #[doc = " only need to store one borrow index."] pub (crate) activation_map : FxIndexMap < Location , Vec < BorrowIndex > > , #[doc = " Map from local to all the borrows on that local."] pub (crate) local_map : FxIndexMap < mir :: Local , FxIndexSet < BorrowIndex > > , pub (crate) locals_state_at_exit : LocalsStateAtExit , }}}
mkitem!{mkimpl!{impl < 'tcx > BorrowSet < 'tcx > { pub fn location_map (& self) -> & FxIndexMap < Location , BorrowData < 'tcx > > { & self . location_map } pub fn activation_map (& self) -> & FxIndexMap < Location , Vec < BorrowIndex > > { & self . activation_map } pub fn local_map (& self) -> & FxIndexMap < mir :: Local , FxIndexSet < BorrowIndex > > { & self . local_map } pub fn locals_state_at_exit (& self) -> & LocalsStateAtExit { & self . locals_state_at_exit } }}}
mkitem!{mkimpl!{impl < 'tcx > Index < BorrowIndex > for BorrowSet < 'tcx > { type Output = BorrowData < 'tcx > ; fn index (& self , index : BorrowIndex) -> & BorrowData < 'tcx > { & self . location_map [index . as_usize ()] } }}}
mkitem!{mkenum!{#[doc = " Location where a two-phase borrow is activated, if a borrow"] #[doc = " is in fact a two-phase borrow."] #[derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum TwoPhaseActivation { NotTwoPhase , NotActivated , ActivatedAt (Location) , }}}
mkitem!{mkstruct!{#[derive (Debug , Clone)] pub struct BorrowData < 'tcx > { #[doc = " Location where the borrow reservation starts."] #[doc = " In many cases, this will be equal to the activation location but not always."] pub (crate) reserve_location : Location , #[doc = " Location where the borrow is activated."] pub (crate) activation_location : TwoPhaseActivation , #[doc = " What kind of borrow this is"] pub (crate) kind : mir :: BorrowKind , #[doc = " The region for which this borrow is live"] pub (crate) region : RegionVid , #[doc = " Place from which we are borrowing"] pub (crate) borrowed_place : mir :: Place < 'tcx > , #[doc = " Place to which the borrow was stored"] pub (crate) assigned_place : mir :: Place < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > BorrowData < 'tcx > { pub fn reserve_location (& self) -> Location { self . reserve_location } pub fn activation_location (& self) -> TwoPhaseActivation { self . activation_location } pub fn kind (& self) -> mir :: BorrowKind { self . kind } pub fn region (& self) -> RegionVid { self . region } pub fn borrowed_place (& self) -> mir :: Place < 'tcx > { self . borrowed_place } pub fn assigned_place (& self) -> mir :: Place < 'tcx > { self . assigned_place } }}}
mkitem!{mkimpl!{impl < 'tcx > fmt :: Display for BorrowData < 'tcx > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let kind = match self . kind { mir :: BorrowKind :: Shared => "" , mir :: BorrowKind :: Fake (mir :: FakeBorrowKind :: Deep) => "fake " , mir :: BorrowKind :: Fake (mir :: FakeBorrowKind :: Shallow) => "fake shallow " , mir :: BorrowKind :: Mut { kind : mir :: MutBorrowKind :: ClosureCapture } => "uniq " , mir :: BorrowKind :: Mut { kind : mir :: MutBorrowKind :: Default | mir :: MutBorrowKind :: TwoPhaseBorrow , } => "mut " , } ; write ! (w , "&{:?} {}{:?}" , self . region , kind , self . borrowed_place) } }}}
mkitem!{mkenum!{pub enum LocalsStateAtExit { AllAreInvalidated , SomeAreInvalidated { has_storage_dead_or_moved : DenseBitSet < Local > } , }}}
mkitem!{mkimpl!{impl LocalsStateAtExit { fn build < 'tcx > (locals_are_invalidated_at_exit : bool , body : & Body < 'tcx > , move_data : & MoveData < 'tcx > ,) -> Self { struct HasStorageDead (DenseBitSet < Local >) ; impl < 'tcx > Visitor < 'tcx > for HasStorageDead { fn visit_local (& mut self , local : Local , ctx : PlaceContext , _ : Location) { if ctx == PlaceContext :: NonUse (NonUseContext :: StorageDead) { self . 0 . insert (local) ; } } } if locals_are_invalidated_at_exit { LocalsStateAtExit :: AllAreInvalidated } else { let mut has_storage_dead = HasStorageDead (DenseBitSet :: new_empty (body . local_decls . len ())) ; has_storage_dead . visit_body (body) ; let mut has_storage_dead_or_moved = has_storage_dead . 0 ; for move_out in & move_data . moves { has_storage_dead_or_moved . insert (move_data . base_local (move_out . path)) ; } LocalsStateAtExit :: SomeAreInvalidated { has_storage_dead_or_moved } } } }}}
mkitem!{mkimpl!{impl < 'tcx > BorrowSet < 'tcx > { pub fn build (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , locals_are_invalidated_at_exit : bool , move_data : & MoveData < 'tcx > ,) -> Self { let mut visitor = GatherBorrows { tcx , body , location_map : Default :: default () , activation_map : Default :: default () , local_map : Default :: default () , pending_activations : Default :: default () , locals_state_at_exit : LocalsStateAtExit :: build (locals_are_invalidated_at_exit , body , move_data ,) , } ; for (block , block_data) in traversal :: preorder (body) { visitor . visit_basic_block_data (block , block_data) ; } BorrowSet { location_map : visitor . location_map , activation_map : visitor . activation_map , local_map : visitor . local_map , locals_state_at_exit : visitor . locals_state_at_exit , } } pub (crate) fn activations_at_location (& self , location : Location) -> & [BorrowIndex] { self . activation_map . get (& location) . map_or (& [] , | activations | & activations [..]) } pub (crate) fn len (& self) -> usize { self . location_map . len () } pub (crate) fn indices (& self) -> impl Iterator < Item = BorrowIndex > { BorrowIndex :: ZERO .. BorrowIndex :: from_usize (self . len ()) } pub (crate) fn iter_enumerated (& self) -> impl Iterator < Item = (BorrowIndex , & BorrowData < 'tcx >) > { self . indices () . zip (self . location_map . values ()) } pub (crate) fn get_index_of (& self , location : & Location) -> Option < BorrowIndex > { self . location_map . get_index_of (location) . map (BorrowIndex :: from) } }}}
mkitem!{mkstruct!{struct GatherBorrows < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , location_map : FxIndexMap < Location , BorrowData < 'tcx > > , activation_map : FxIndexMap < Location , Vec < BorrowIndex > > , local_map : FxIndexMap < mir :: Local , FxIndexSet < BorrowIndex > > , #[doc = " When we encounter a 2-phase borrow statement, it will always"] #[doc = " be assigning into a temporary TEMP:"] #[doc = ""] #[doc = "    TEMP = &foo"] #[doc = ""] #[doc = " We add TEMP into this map with `b`, where `b` is the index of"] #[doc = " the borrow. When we find a later use of this activation, we"] #[doc = " remove from the map (and add to the \"tombstone\" set below)."] pending_activations : FxIndexMap < mir :: Local , BorrowIndex > , locals_state_at_exit : LocalsStateAtExit , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Visitor < 'tcx > for GatherBorrows < 'a , 'tcx > { fn visit_assign (& mut self , assigned_place : & mir :: Place < 'tcx > , rvalue : & mir :: Rvalue < 'tcx > , location : mir :: Location ,) { if let & mir :: Rvalue :: Ref (region , kind , borrowed_place) = rvalue { if borrowed_place . ignore_borrow (self . tcx , self . body , & self . locals_state_at_exit) { debug ! ("ignoring_borrow of {:?}" , borrowed_place) ; return ; } let region = region . as_var () ; let borrow = BorrowData { kind , region , reserve_location : location , activation_location : TwoPhaseActivation :: NotTwoPhase , borrowed_place , assigned_place : * assigned_place , } ; let (idx , _) = self . location_map . insert_full (location , borrow) ; let idx = BorrowIndex :: from (idx) ; self . insert_as_pending_if_two_phase (location , assigned_place , kind , idx) ; self . local_map . entry (borrowed_place . local) . or_default () . insert (idx) ; } self . super_assign (assigned_place , rvalue , location) } fn visit_local (& mut self , temp : Local , context : PlaceContext , location : Location) { if ! context . is_use () { return ; } if let Some (& borrow_index) = self . pending_activations . get (& temp) { let borrow_data = & mut self . location_map [borrow_index . as_usize ()] ; if borrow_data . reserve_location == location && context == PlaceContext :: MutatingUse (MutatingUseContext :: Store) { return ; } if let TwoPhaseActivation :: ActivatedAt (other_location) = borrow_data . activation_location { span_bug ! (self . body . source_info (location) . span , "found two uses for 2-phase borrow temporary {:?}: \
                     {:?} and {:?}" , temp , location , other_location ,) ; } assert_eq ! (borrow_data . activation_location , TwoPhaseActivation :: NotActivated , "never found an activation for this borrow!" ,) ; self . activation_map . entry (location) . or_default () . push (borrow_index) ; borrow_data . activation_location = TwoPhaseActivation :: ActivatedAt (location) ; } } fn visit_rvalue (& mut self , rvalue : & mir :: Rvalue < 'tcx > , location : mir :: Location) { if let & mir :: Rvalue :: Ref (region , kind , place) = rvalue { let borrow_data = & self . location_map [& location] ; assert_eq ! (borrow_data . reserve_location , location) ; assert_eq ! (borrow_data . kind , kind) ; assert_eq ! (borrow_data . region , region . as_var ()) ; assert_eq ! (borrow_data . borrowed_place , place) ; } self . super_rvalue (rvalue , location) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > GatherBorrows < 'a , 'tcx > { #[doc = " If this is a two-phase borrow, then we will record it"] #[doc = " as \"pending\" until we find the activating use."] fn insert_as_pending_if_two_phase (& mut self , start_location : Location , assigned_place : & mir :: Place < 'tcx > , kind : mir :: BorrowKind , borrow_index : BorrowIndex ,) { debug ! ("Borrows::insert_as_pending_if_two_phase({:?}, {:?}, {:?})" , start_location , assigned_place , borrow_index ,) ; if ! kind . allows_two_phase_borrow () { debug ! ("  -> {:?}" , start_location) ; return ; } let Some (temp) = assigned_place . as_local () else { span_bug ! (self . body . source_info (start_location) . span , "expected 2-phase borrow to assign to a local, not `{:?}`" , assigned_place ,) ; } ; { let borrow_data = & mut self . location_map [borrow_index . as_usize ()] ; borrow_data . activation_location = TwoPhaseActivation :: NotActivated ; } let old_value = self . pending_activations . insert (temp , borrow_index) ; if let Some (old_index) = old_value { span_bug ! (self . body . source_info (start_location) . span , "found already pending activation for temp: {:?} \
                       at borrow_index: {:?} with associated data {:?}" , temp , old_index , self . location_map [old_index . as_usize ()]) ; } } }}}