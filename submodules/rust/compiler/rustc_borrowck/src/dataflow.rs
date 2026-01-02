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
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_index :: bit_set :: { DenseBitSet , MixedBitSet } ;}
mkuse!{use rustc_middle :: mir :: { self , BasicBlock , Body , CallReturnPlaces , Location , Place , TerminatorEdges , } ;}
mkuse!{use rustc_middle :: ty :: { RegionVid , TyCtxt } ;}
mkuse!{use rustc_mir_dataflow :: fmt :: DebugWithContext ;}
mkuse!{use rustc_mir_dataflow :: impls :: { EverInitializedPlaces , EverInitializedPlacesDomain , MaybeUninitializedPlaces , MaybeUninitializedPlacesDomain , } ;}
mkuse!{use rustc_mir_dataflow :: { Analysis , GenKill , JoinSemiLattice } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: { BorrowSet , PlaceConflictBias , PlaceExt , RegionInferenceContext , places_conflict } ;}
mkitem!{mkstruct!{pub (crate) struct Borrowck < 'a , 'tcx > { pub (crate) borrows : Borrows < 'a , 'tcx > , pub (crate) uninits : MaybeUninitializedPlaces < 'a , 'tcx > , pub (crate) ever_inits : EverInitializedPlaces < 'a , 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Analysis < 'tcx > for Borrowck < 'a , 'tcx > { type Domain = BorrowckDomain ; const NAME : & 'static str = "borrowck" ; fn bottom_value (& self , body : & mir :: Body < 'tcx >) -> Self :: Domain { BorrowckDomain { borrows : self . borrows . bottom_value (body) , uninits : self . uninits . bottom_value (body) , ever_inits : self . ever_inits . bottom_value (body) , } } fn initialize_start_block (& self , _body : & mir :: Body < 'tcx > , _state : & mut Self :: Domain) { unreachable ! () ; } fn apply_early_statement_effect (& mut self , state : & mut Self :: Domain , stmt : & mir :: Statement < 'tcx > , loc : Location ,) { self . borrows . apply_early_statement_effect (& mut state . borrows , stmt , loc) ; self . uninits . apply_early_statement_effect (& mut state . uninits , stmt , loc) ; self . ever_inits . apply_early_statement_effect (& mut state . ever_inits , stmt , loc) ; } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , stmt : & mir :: Statement < 'tcx > , loc : Location ,) { self . borrows . apply_primary_statement_effect (& mut state . borrows , stmt , loc) ; self . uninits . apply_primary_statement_effect (& mut state . uninits , stmt , loc) ; self . ever_inits . apply_primary_statement_effect (& mut state . ever_inits , stmt , loc) ; } fn apply_early_terminator_effect (& mut self , state : & mut Self :: Domain , term : & mir :: Terminator < 'tcx > , loc : Location ,) { self . borrows . apply_early_terminator_effect (& mut state . borrows , term , loc) ; self . uninits . apply_early_terminator_effect (& mut state . uninits , term , loc) ; self . ever_inits . apply_early_terminator_effect (& mut state . ever_inits , term , loc) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , term : & 'mir mir :: Terminator < 'tcx > , loc : Location ,) -> TerminatorEdges < 'mir , 'tcx > { self . borrows . apply_primary_terminator_effect (& mut state . borrows , term , loc) ; self . uninits . apply_primary_terminator_effect (& mut state . uninits , term , loc) ; self . ever_inits . apply_primary_terminator_effect (& mut state . ever_inits , term , loc) ; TerminatorEdges :: None } fn apply_call_return_effect (& mut self , _state : & mut Self :: Domain , _block : BasicBlock , _return_places : CallReturnPlaces < '_ , 'tcx > ,) { unreachable ! () ; } }}}
mkitem!{mkimpl!{impl JoinSemiLattice for BorrowckDomain { fn join (& mut self , _other : & Self) -> bool { unreachable ! () ; } }}}
mkitem!{mkimpl!{impl < 'tcx , C > DebugWithContext < C > for BorrowckDomain where C : rustc_mir_dataflow :: move_paths :: HasMoveData < 'tcx > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("borrows: ") ? ; self . borrows . fmt_with (ctxt , f) ? ; f . write_str (" uninits: ") ? ; self . uninits . fmt_with (ctxt , f) ? ; f . write_str (" ever_inits: ") ? ; self . ever_inits . fmt_with (ctxt , f) ? ; Ok (()) } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self == old { return Ok (()) ; } if self . borrows != old . borrows { f . write_str ("borrows: ") ? ; self . borrows . fmt_diff_with (& old . borrows , ctxt , f) ? ; f . write_str ("\n") ? ; } if self . uninits != old . uninits { f . write_str ("uninits: ") ? ; self . uninits . fmt_diff_with (& old . uninits , ctxt , f) ? ; f . write_str ("\n") ? ; } if self . ever_inits != old . ever_inits { f . write_str ("ever_inits: ") ? ; self . ever_inits . fmt_diff_with (& old . ever_inits , ctxt , f) ? ; f . write_str ("\n") ? ; } Ok (()) } }}}
mkitem!{mkstruct!{#[doc = " The transient state of the dataflow analyses used by the borrow checker."] #[derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct BorrowckDomain { pub (crate) borrows : BorrowsDomain , pub (crate) uninits : MaybeUninitializedPlacesDomain , pub (crate) ever_inits : EverInitializedPlacesDomain , }}}
mkitem!{rustc_index :: newtype_index ! { #[orderable] #[debug_format = "bw{}"] pub struct BorrowIndex { } }}
mkitem!{mkstruct!{#[doc = " `Borrows` stores the data used in the analyses that track the flow"] #[doc = " of borrows."] #[doc = ""] #[doc = " It uniquely identifies every borrow (`Rvalue::Ref`) by a"] #[doc = " `BorrowIndex`, and maps each such index to a `BorrowData`"] #[doc = " describing the borrow. These indexes are used for representing the"] #[doc = " borrows in compact bitvectors."] pub struct Borrows < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , borrow_set : & 'a BorrowSet < 'tcx > , borrows_out_of_scope_at_location : FxIndexMap < Location , Vec < BorrowIndex > > , }}}
mkitem!{mkstruct!{struct OutOfScopePrecomputer < 'a , 'tcx > { visited : DenseBitSet < mir :: BasicBlock > , visit_stack : Vec < mir :: BasicBlock > , body : & 'a Body < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , borrows_out_of_scope_at_location : FxIndexMap < Location , Vec < BorrowIndex > > , }}}
mkitem!{mkimpl!{impl < 'tcx > OutOfScopePrecomputer < '_ , 'tcx > { fn compute (body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , borrow_set : & BorrowSet < 'tcx > ,) -> FxIndexMap < Location , Vec < BorrowIndex > > { let mut prec = OutOfScopePrecomputer { visited : DenseBitSet :: new_empty (body . basic_blocks . len ()) , visit_stack : vec ! [] , body , regioncx , borrows_out_of_scope_at_location : FxIndexMap :: default () , } ; for (borrow_index , borrow_data) in borrow_set . iter_enumerated () { let borrow_region = borrow_data . region ; let location = borrow_data . reserve_location ; prec . precompute_borrows_out_of_scope (borrow_index , borrow_region , location) ; } prec . borrows_out_of_scope_at_location } fn precompute_borrows_out_of_scope (& mut self , borrow_index : BorrowIndex , borrow_region : RegionVid , first_location : Location ,) { let first_block = first_location . block ; let first_bb_data = & self . body . basic_blocks [first_block] ; let first_lo = first_location . statement_index ; let first_hi = first_bb_data . statements . len () ; if let Some (kill_stmt) = self . regioncx . first_non_contained_inclusive (borrow_region , first_block , first_lo , first_hi ,) { let kill_location = Location { block : first_block , statement_index : kill_stmt } ; debug ! ("borrow {:?} gets killed at {:?}" , borrow_index , kill_location) ; self . borrows_out_of_scope_at_location . entry (kill_location) . or_default () . push (borrow_index) ; return ; } for succ_bb in first_bb_data . terminator () . successors () { if self . visited . insert (succ_bb) { self . visit_stack . push (succ_bb) ; } } while let Some (block) = self . visit_stack . pop () { let bb_data = & self . body [block] ; let num_stmts = bb_data . statements . len () ; if let Some (kill_stmt) = self . regioncx . first_non_contained_inclusive (borrow_region , block , 0 , num_stmts) { let kill_location = Location { block , statement_index : kill_stmt } ; debug ! ("borrow {:?} gets killed at {:?}" , borrow_index , kill_location) ; self . borrows_out_of_scope_at_location . entry (kill_location) . or_default () . push (borrow_index) ; continue ; } for succ_bb in bb_data . terminator () . successors () { if self . visited . insert (succ_bb) { self . visit_stack . push (succ_bb) ; } } } self . visited . clear () ; } }}}

macro_rules! calculate_borrows_out_of_scope_at_location_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function calculate_borrows_out_of_scope_at_location in module {}", module_path!());
    };
}

mkfn!{
    calculate_borrows_out_of_scope_at_location_introspect!();
    pub fn calculate_borrows_out_of_scope_at_location < 'tcx > (body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , borrow_set : & BorrowSet < 'tcx > ,) -> FxIndexMap < Location , Vec < BorrowIndex > > { OutOfScopePrecomputer :: compute (body , regioncx , borrow_set) }
}
mkitem!{mkstruct!{struct PoloniusOutOfScopePrecomputer < 'a , 'tcx > { visited : DenseBitSet < mir :: BasicBlock > , visit_stack : Vec < mir :: BasicBlock > , body : & 'a Body < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , loans_out_of_scope_at_location : FxIndexMap < Location , Vec < BorrowIndex > > , }}}
mkitem!{mkimpl!{impl < 'tcx > PoloniusOutOfScopePrecomputer < '_ , 'tcx > { fn compute (body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , borrow_set : & BorrowSet < 'tcx > ,) -> FxIndexMap < Location , Vec < BorrowIndex > > { let mut prec = PoloniusOutOfScopePrecomputer { visited : DenseBitSet :: new_empty (body . basic_blocks . len ()) , visit_stack : vec ! [] , body , regioncx , loans_out_of_scope_at_location : FxIndexMap :: default () , } ; for (loan_idx , loan_data) in borrow_set . iter_enumerated () { let loan_issued_at = loan_data . reserve_location ; prec . precompute_loans_out_of_scope (loan_idx , loan_issued_at) ; } prec . loans_out_of_scope_at_location } #[doc = " Loans are in scope while they are live: whether they are contained within any live region."] #[doc = " In the location-insensitive analysis, a loan will be contained in a region if the issuing"] #[doc = " region can reach it in the subset graph. So this is a reachability problem."] fn precompute_loans_out_of_scope (& mut self , loan_idx : BorrowIndex , loan_issued_at : Location) { let first_block = loan_issued_at . block ; let first_bb_data = & self . body . basic_blocks [first_block] ; let first_lo = loan_issued_at . statement_index ; let first_hi = first_bb_data . statements . len () ; if let Some (kill_location) = self . loan_kill_location (loan_idx , loan_issued_at , first_block , first_lo , first_hi) { debug ! ("loan {:?} gets killed at {:?}" , loan_idx , kill_location) ; self . loans_out_of_scope_at_location . entry (kill_location) . or_default () . push (loan_idx) ; return ; } for succ_bb in first_bb_data . terminator () . successors () { if self . visited . insert (succ_bb) { self . visit_stack . push (succ_bb) ; } } while let Some (block) = self . visit_stack . pop () { let bb_data = & self . body [block] ; let num_stmts = bb_data . statements . len () ; if let Some (kill_location) = self . loan_kill_location (loan_idx , loan_issued_at , block , 0 , num_stmts) { debug ! ("loan {:?} gets killed at {:?}" , loan_idx , kill_location) ; self . loans_out_of_scope_at_location . entry (kill_location) . or_default () . push (loan_idx) ; continue ; } for succ_bb in bb_data . terminator () . successors () { if self . visited . insert (succ_bb) { self . visit_stack . push (succ_bb) ; } } } self . visited . clear () ; assert ! (self . visit_stack . is_empty () , "visit stack should be empty") ; } #[doc = " Returns the lowest statement in `start..=end`, where the loan goes out of scope, if any."] #[doc = " This is the statement where the issuing region can't reach any of the regions that are live"] #[doc = " at this point."] fn loan_kill_location (& self , loan_idx : BorrowIndex , loan_issued_at : Location , block : BasicBlock , start : usize , end : usize ,) -> Option < Location > { for statement_index in start ..= end { let location = Location { block , statement_index } ; if location == loan_issued_at { continue ; } if self . regioncx . is_loan_live_at (loan_idx , location) { continue ; } return Some (location) ; } None } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Borrows < 'a , 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , borrow_set : & 'a BorrowSet < 'tcx > ,) -> Self { let borrows_out_of_scope_at_location = if ! tcx . sess . opts . unstable_opts . polonius . is_next_enabled () { calculate_borrows_out_of_scope_at_location (body , regioncx , borrow_set) } else { PoloniusOutOfScopePrecomputer :: compute (body , regioncx , borrow_set) } ; Borrows { tcx , body , borrow_set , borrows_out_of_scope_at_location } } #[doc = " Add all borrows to the kill set, if those borrows are out of scope at `location`."] #[doc = " That means they went out of a nonlexical scope"] fn kill_loans_out_of_scope_at_location (& self , state : & mut < Self as Analysis < 'tcx > > :: Domain , location : Location ,) { if let Some (indices) = self . borrows_out_of_scope_at_location . get (& location) { state . kill_all (indices . iter () . copied ()) ; } } #[doc = " Kill any borrows that conflict with `place`."] fn kill_borrows_on_place (& self , state : & mut < Self as Analysis < 'tcx > > :: Domain , place : Place < 'tcx > ,) { debug ! ("kill_borrows_on_place: place={:?}" , place) ; let other_borrows_of_local = self . borrow_set . local_map . get (& place . local) . into_iter () . flat_map (| bs | bs . iter ()) . copied () ; if place . projection . is_empty () { if ! self . body . local_decls [place . local] . is_ref_to_static () { state . kill_all (other_borrows_of_local) ; } return ; } let definitely_conflicting_borrows = other_borrows_of_local . filter (| & i | { places_conflict (self . tcx , self . body , self . borrow_set [i] . borrowed_place , place , PlaceConflictBias :: NoOverlap ,) }) ; state . kill_all (definitely_conflicting_borrows) ; } }}}
mkitem!{type BorrowsDomain = MixedBitSet < BorrowIndex > ;}
mkitem!{mkimpl!{#[doc = " Forward dataflow computation of the set of borrows that are in scope at a particular location."] #[doc = " - we gen the introduced loans"] #[doc = " - we kill loans on locals going out of (regular) scope"] #[doc = " - we kill the loans going out of their region's NLL scope: in NLL terms, the frontier where a"] #[doc = "   region stops containing the CFG points reachable from the issuing location."] #[doc = " - we also kill loans of conflicting places when overwriting a shared path: e.g. borrows of"] #[doc = "   `a.b.c` when `a` is overwritten."] impl < 'tcx > rustc_mir_dataflow :: Analysis < 'tcx > for Borrows < '_ , 'tcx > { type Domain = BorrowsDomain ; const NAME : & 'static str = "borrows" ; fn bottom_value (& self , _ : & mir :: Body < 'tcx >) -> Self :: Domain { MixedBitSet :: new_empty (self . borrow_set . len ()) } fn initialize_start_block (& self , _ : & mir :: Body < 'tcx > , _ : & mut Self :: Domain) { } fn apply_early_statement_effect (& mut self , state : & mut Self :: Domain , _statement : & mir :: Statement < 'tcx > , location : Location ,) { self . kill_loans_out_of_scope_at_location (state , location) ; } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , stmt : & mir :: Statement < 'tcx > , location : Location ,) { match & stmt . kind { mir :: StatementKind :: Assign (box (lhs , rhs)) => { if let mir :: Rvalue :: Ref (_ , _ , place) = rhs { if place . ignore_borrow (self . tcx , self . body , & self . borrow_set . locals_state_at_exit ,) { return ; } let index = self . borrow_set . get_index_of (& location) . unwrap_or_else (| | { panic ! ("could not find BorrowIndex for location {location:?}") ; }) ; state . gen_ (index) ; } self . kill_borrows_on_place (state , * lhs) ; } mir :: StatementKind :: StorageDead (local) => { self . kill_borrows_on_place (state , Place :: from (* local)) ; } mir :: StatementKind :: FakeRead (..) | mir :: StatementKind :: SetDiscriminant { .. } | mir :: StatementKind :: Deinit (..) | mir :: StatementKind :: StorageLive (..) | mir :: StatementKind :: Retag { .. } | mir :: StatementKind :: PlaceMention (..) | mir :: StatementKind :: AscribeUserType (..) | mir :: StatementKind :: Coverage (..) | mir :: StatementKind :: Intrinsic (..) | mir :: StatementKind :: ConstEvalCounter | mir :: StatementKind :: BackwardIncompatibleDropHint { .. } | mir :: StatementKind :: Nop => { } } } fn apply_early_terminator_effect (& mut self , state : & mut Self :: Domain , _terminator : & mir :: Terminator < 'tcx > , location : Location ,) { self . kill_loans_out_of_scope_at_location (state , location) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , _location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { if let mir :: TerminatorKind :: InlineAsm { operands , .. } = & terminator . kind { for op in operands { if let mir :: InlineAsmOperand :: Out { place : Some (place) , .. } | mir :: InlineAsmOperand :: InOut { out_place : Some (place) , .. } = * op { self . kill_borrows_on_place (state , place) ; } } } terminator . edges () } }}}
mkitem!{mkimpl!{impl < C > DebugWithContext < C > for BorrowIndex { }}}