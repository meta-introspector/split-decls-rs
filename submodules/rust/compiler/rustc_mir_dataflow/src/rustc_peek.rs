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
mkuse!{use rustc_ast :: MetaItem ;}
mkuse!{use rustc_middle :: mir :: { self , Body , Local , Location } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use tracing :: { debug , info } ;}
mkuse!{use crate :: errors :: { PeekArgumentNotALocal , PeekArgumentUntracked , PeekBitNotSet , PeekMustBeNotTemporary , PeekMustBePlaceOrRefPlace , StopAfterDataFlowEndedCompilation , } ;}
mkuse!{use crate :: framework :: BitSetExt ;}
mkuse!{use crate :: impls :: { MaybeInitializedPlaces , MaybeLiveLocals , MaybeUninitializedPlaces } ;}
mkuse!{use crate :: move_paths :: { HasMoveData , LookupResult , MoveData , MovePathIndex } ;}
mkuse!{use crate :: { Analysis , JoinSemiLattice , ResultsCursor } ;}

macro_rules! has_rustc_mir_with_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_rustc_mir_with in module {}", module_path!());
    };
}

mkfn!{
    has_rustc_mir_with_introspect!();
    fn has_rustc_mir_with (tcx : TyCtxt < '_ > , def_id : DefId , name : Symbol) -> Option < MetaItem > { for attr in tcx . get_attrs (def_id , sym :: rustc_mir) { let items = attr . meta_item_list () ; for item in items . iter () . flat_map (| l | l . iter ()) { match item . meta_item () { Some (mi) if mi . has_name (name) => return Some (mi . clone ()) , _ => continue , } } } None }
}

macro_rules! sanity_check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_introspect!();
    pub fn sanity_check < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { let def_id = body . source . def_id () ; if ! tcx . has_attr (def_id , sym :: rustc_mir) { debug ! ("skipping rustc_peek::SanityCheck on {}" , tcx . def_path_str (def_id)) ; return ; } else { debug ! ("running rustc_peek::SanityCheck on {}" , tcx . def_path_str (def_id)) ; } let move_data = MoveData :: gather_moves (body , tcx , | _ | true) ; if has_rustc_mir_with (tcx , def_id , sym :: rustc_peek_maybe_init) . is_some () { let flow_inits = MaybeInitializedPlaces :: new (tcx , body , & move_data) . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; sanity_check_via_rustc_peek (tcx , flow_inits) ; } if has_rustc_mir_with (tcx , def_id , sym :: rustc_peek_maybe_uninit) . is_some () { let flow_uninits = MaybeUninitializedPlaces :: new (tcx , body , & move_data) . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; sanity_check_via_rustc_peek (tcx , flow_uninits) ; } if has_rustc_mir_with (tcx , def_id , sym :: rustc_peek_liveness) . is_some () { let flow_liveness = MaybeLiveLocals . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; sanity_check_via_rustc_peek (tcx , flow_liveness) ; } if has_rustc_mir_with (tcx , def_id , sym :: stop_after_dataflow) . is_some () { tcx . dcx () . emit_fatal (StopAfterDataFlowEndedCompilation) ; } }
}

macro_rules! sanity_check_via_rustc_peek_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_via_rustc_peek in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_via_rustc_peek_introspect!();
    #[doc = " This function scans `mir` for all calls to the intrinsic"] #[doc = " `rustc_peek` that have the expression form `rustc_peek(&expr)`."] #[doc = ""] #[doc = " For each such call, determines what the dataflow bit-state is for"] #[doc = " the L-value corresponding to `expr`; if the bit-state is a 1, then"] #[doc = " that call to `rustc_peek` is ignored by the sanity check. If the"] #[doc = " bit-state is a 0, then this pass emits an error message saying"] #[doc = " \"rustc_peek: bit not set\"."] #[doc = ""] #[doc = " The intention is that one can write unit tests for dataflow by"] #[doc = " putting code into a UI test and using `rustc_peek` to"] #[doc = " make observations about the results of dataflow static analyses."] #[doc = ""] #[doc = " (If there are any calls to `rustc_peek` that do not match the"] #[doc = " expression form above, then that emits an error as well, but those"] #[doc = " errors are not intended to be used for unit tests.)"] fn sanity_check_via_rustc_peek < 'tcx , A > (tcx : TyCtxt < 'tcx > , mut cursor : ResultsCursor < '_ , 'tcx , A >) where A : RustcPeekAt < 'tcx > , { let def_id = cursor . body () . source . def_id () ; debug ! ("sanity_check_via_rustc_peek def_id: {:?}" , def_id) ; let peek_calls = cursor . body () . basic_blocks . iter_enumerated () . filter_map (| (bb , block_data) | { PeekCall :: from_terminator (tcx , block_data . terminator ()) . map (| call | (bb , block_data , call)) }) ; for (bb , block_data , call) in peek_calls { let (statement_index , peek_rval) = block_data . statements . iter () . enumerate () . find_map (| (i , stmt) | value_assigned_to_local (stmt , call . arg) . map (| rval | (i , rval))) . expect ("call to rustc_peek should be preceded by \
                    assignment to temporary holding its argument" ,) ; match (call . kind , peek_rval) { (PeekCallKind :: ByRef , mir :: Rvalue :: Ref (_ , _ , place)) | (PeekCallKind :: ByVal , mir :: Rvalue :: Use (mir :: Operand :: Move (place) | mir :: Operand :: Copy (place)) ,) => { let loc = Location { block : bb , statement_index } ; cursor . seek_before_primary_effect (loc) ; let state = cursor . get () ; let analysis = cursor . analysis () ; analysis . peek_at (tcx , * place , state , call) ; } _ => { tcx . dcx () . emit_err (PeekMustBePlaceOrRefPlace { span : call . span }) ; } } } }
}

macro_rules! value_assigned_to_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function value_assigned_to_local in module {}", module_path!());
    };
}

mkfn!{
    value_assigned_to_local_introspect!();
    #[doc = " If `stmt` is an assignment where the LHS is the given local (with no projections), returns the"] #[doc = " RHS of the assignment."] fn value_assigned_to_local < 'a , 'tcx > (stmt : & 'a mir :: Statement < 'tcx > , local : Local ,) -> Option < & 'a mir :: Rvalue < 'tcx > > { if let mir :: StatementKind :: Assign (box (place , rvalue)) = & stmt . kind && let Some (l) = place . as_local () && local == l { return Some (& * rvalue) ; } None }
}
mkitem!{mkenum!{#[derive (Clone , Copy , Debug)] enum PeekCallKind { ByVal , ByRef , }}}
mkitem!{mkimpl!{impl PeekCallKind { fn from_arg_ty (arg : Ty < '_ >) -> Self { match arg . kind () { ty :: Ref (_ , _ , _) => PeekCallKind :: ByRef , _ => PeekCallKind :: ByVal , } } }}}
mkitem!{mkstruct!{#[derive (Clone , Copy , Debug)] struct PeekCall { arg : Local , kind : PeekCallKind , span : Span , }}}
mkitem!{mkimpl!{impl PeekCall { fn from_terminator < 'tcx > (tcx : TyCtxt < 'tcx > , terminator : & mir :: Terminator < 'tcx > ,) -> Option < Self > { use mir :: Operand ; let span = terminator . source_info . span ; if let mir :: TerminatorKind :: Call { func : Operand :: Constant (func) , args , .. } = & terminator . kind && let ty :: FnDef (def_id , fn_args) = * func . const_ . ty () . kind () { if tcx . intrinsic (def_id) ? . name != sym :: rustc_peek { return None ; } assert_eq ! (fn_args . len () , 1) ; let kind = PeekCallKind :: from_arg_ty (fn_args . type_at (0)) ; let arg = match & args [0] . node { Operand :: Copy (place) | Operand :: Move (place) => { if let Some (local) = place . as_local () { local } else { tcx . dcx () . emit_err (PeekMustBeNotTemporary { span }) ; return None ; } } _ => { tcx . dcx () . emit_err (PeekMustBeNotTemporary { span }) ; return None ; } } ; return Some (PeekCall { arg , kind , span }) ; } None } }}}
mkitem!{mktrait!{trait RustcPeekAt < 'tcx > : Analysis < 'tcx > { fn peek_at (& self , tcx : TyCtxt < 'tcx > , place : mir :: Place < 'tcx > , state : & Self :: Domain , call : PeekCall ,) ; }}}
mkitem!{mkimpl!{impl < 'tcx , A , D > RustcPeekAt < 'tcx > for A where A : Analysis < 'tcx , Domain = D > + HasMoveData < 'tcx > , D : JoinSemiLattice + Clone + BitSetExt < MovePathIndex > , { fn peek_at (& self , tcx : TyCtxt < 'tcx > , place : mir :: Place < 'tcx > , state : & Self :: Domain , call : PeekCall ,) { match self . move_data () . rev_lookup . find (place . as_ref ()) { LookupResult :: Exact (peek_mpi) => { let bit_state = state . contains (peek_mpi) ; debug ! ("rustc_peek({:?} = &{:?}) bit_state: {}" , call . arg , place , bit_state) ; if ! bit_state { tcx . dcx () . emit_err (PeekBitNotSet { span : call . span }) ; } } LookupResult :: Parent (..) => { tcx . dcx () . emit_err (PeekArgumentUntracked { span : call . span }) ; } } } }}}
mkitem!{mkimpl!{impl < 'tcx > RustcPeekAt < 'tcx > for MaybeLiveLocals { fn peek_at (& self , tcx : TyCtxt < 'tcx > , place : mir :: Place < 'tcx > , state : & Self :: Domain , call : PeekCall ,) { info ! (? place , "peek_at") ; let Some (local) = place . as_local () else { tcx . dcx () . emit_err (PeekArgumentNotALocal { span : call . span }) ; return ; } ; if ! state . contains (local) { tcx . dcx () . emit_err (PeekBitNotSet { span : call . span }) ; } } }}}