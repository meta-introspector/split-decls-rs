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
mkuse!{use rustc_abi :: { FieldIdx , VariantIdx } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt } ;}
mkuse!{use rustc_mir_dataflow :: impls :: { MaybeInitializedPlaces , MaybeUninitializedPlaces } ;}
mkuse!{use rustc_mir_dataflow :: move_paths :: { LookupResult , MoveData , MovePathIndex } ;}
mkuse!{use rustc_mir_dataflow :: { Analysis , DropFlagState , MoveDataTypingEnv , ResultsCursor , on_all_children_bits , on_lookup_result_bits , } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: deref_separator :: deref_finder ;}
mkuse!{use crate :: elaborate_drop :: { DropElaborator , DropFlagMode , DropStyle , Unwind , elaborate_drop } ;}
mkuse!{use crate :: patch :: MirPatch ;}
mkitem!{mkstruct!{#[doc = " During MIR building, Drop terminators are inserted in every place where a drop may occur."] #[doc = " However, in this phase, the presence of these terminators does not guarantee that a destructor"] #[doc = " will run, as the target of the drop may be uninitialized."] #[doc = " In general, the compiler cannot determine at compile time whether a destructor will run or not."] #[doc = ""] #[doc = " At a high level, this pass refines Drop to only run the destructor if the"] #[doc = " target is initialized. The way this is achieved is by inserting drop flags for every variable"] #[doc = " that may be dropped, and then using those flags to determine whether a destructor should run."] #[doc = " Once this is complete, Drop terminators in the MIR correspond to a call to the \"drop glue\" or"] #[doc = " \"drop shim\" for the type of the dropped place."] #[doc = ""] #[doc = " This pass relies on dropped places having an associated move path, which is then used to"] #[doc = " determine the initialization status of the place and its descendants."] #[doc = " It's worth noting that a MIR containing a Drop without an associated move path is probably ill"] #[doc = " formed, as it would allow running a destructor on a place behind a reference:"] #[doc = ""] #[doc = " ```text"] #[doc = " fn drop_term<T>(t: &mut T) {"] #[doc = "     mir! {"] #[doc = "         {"] #[doc = "             Drop(*t, exit)"] #[doc = "         }"] #[doc = "         exit = {"] #[doc = "             Return()"] #[doc = "         }"] #[doc = "     }"] #[doc = " }"] #[doc = " ```"] pub (super) struct ElaborateDrops ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for ElaborateDrops { #[instrument (level = "trace" , skip (self , tcx , body))] fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { debug ! ("elaborate_drops({:?} @ {:?})" , body . source , body . span) ; let typing_env = ty :: TypingEnv :: post_analysis (tcx , body . source . def_id ()) ; let move_data = MoveData :: gather_moves (body , tcx , | ty | ty . needs_drop (tcx , typing_env)) ; let elaborate_patch = { let env = MoveDataTypingEnv { move_data , typing_env } ; let mut inits = MaybeInitializedPlaces :: new (tcx , body , & env . move_data) . exclude_inactive_in_otherwise () . skipping_unreachable_unwind () . iterate_to_fixpoint (tcx , body , Some ("elaborate_drops")) . into_results_cursor (body) ; let dead_unwinds = compute_dead_unwinds (body , & mut inits) ; let uninits = MaybeUninitializedPlaces :: new (tcx , body , & env . move_data) . include_inactive_in_otherwise () . mark_inactive_variants_as_uninit () . skipping_unreachable_unwind (dead_unwinds) . iterate_to_fixpoint (tcx , body , Some ("elaborate_drops")) . into_results_cursor (body) ; let drop_flags = IndexVec :: from_elem (None , & env . move_data . move_paths) ; ElaborateDropsCtxt { tcx , body , env : & env , init_data : InitializationData { inits , uninits } , drop_flags , patch : MirPatch :: new (body) , } . elaborate () } ; elaborate_patch . apply (body) ; deref_finder (tcx , body) ; } fn is_required (& self) -> bool { true } }}}

macro_rules! compute_dead_unwinds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_dead_unwinds in module {}", module_path!());
    };
}

mkfn!{
    compute_dead_unwinds_introspect!();
    #[doc = " Records unwind edges which are known to be unreachable, because they are in `drop` terminators"] #[doc = " that can't drop anything."] #[instrument (level = "trace" , skip (body , flow_inits) , ret)] fn compute_dead_unwinds < 'a , 'tcx > (body : & 'a Body < 'tcx > , flow_inits : & mut ResultsCursor < 'a , 'tcx , MaybeInitializedPlaces < 'a , 'tcx > > ,) -> DenseBitSet < BasicBlock > { let mut dead_unwinds = DenseBitSet :: new_empty (body . basic_blocks . len ()) ; for (bb , bb_data) in body . basic_blocks . iter_enumerated () { let TerminatorKind :: Drop { place , unwind : UnwindAction :: Cleanup (_) , .. } = bb_data . terminator () . kind else { continue ; } ; flow_inits . seek_before_primary_effect (body . terminator_loc (bb)) ; if flow_inits . analysis () . is_unwind_dead (place , flow_inits . get ()) { dead_unwinds . insert (bb) ; } } dead_unwinds }
}
mkitem!{mkstruct!{struct InitializationData < 'a , 'tcx > { inits : ResultsCursor < 'a , 'tcx , MaybeInitializedPlaces < 'a , 'tcx > > , uninits : ResultsCursor < 'a , 'tcx , MaybeUninitializedPlaces < 'a , 'tcx > > , }}}
mkitem!{mkimpl!{impl InitializationData < '_ , '_ > { fn seek_before (& mut self , loc : Location) { self . inits . seek_before_primary_effect (loc) ; self . uninits . seek_before_primary_effect (loc) ; } fn maybe_init_uninit (& self , path : MovePathIndex) -> (bool , bool) { (self . inits . get () . contains (path) , self . uninits . get () . contains (path)) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > DropElaborator < 'a , 'tcx > for ElaborateDropsCtxt < 'a , 'tcx > { type Path = MovePathIndex ; fn patch_ref (& self) -> & MirPatch < 'tcx > { & self . patch } fn patch (& mut self) -> & mut MirPatch < 'tcx > { & mut self . patch } fn body (& self) -> & 'a Body < 'tcx > { self . body } fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . env . typing_env } fn allow_async_drops (& self) -> bool { true } fn terminator_loc (& self , bb : BasicBlock) -> Location { self . patch . terminator_loc (self . body , bb) } #[instrument (level = "debug" , skip (self) , ret)] fn drop_style (& self , path : Self :: Path , mode : DropFlagMode) -> DropStyle { let ((maybe_init , maybe_uninit) , multipart) = match mode { DropFlagMode :: Shallow => (self . init_data . maybe_init_uninit (path) , false) , DropFlagMode :: Deep => { let mut some_maybe_init = false ; let mut some_maybe_uninit = false ; let mut children_count = 0 ; on_all_children_bits (self . move_data () , path , | child | { let (maybe_init , maybe_uninit) = self . init_data . maybe_init_uninit (child) ; debug ! ("elaborate_drop: state({:?}) = {:?}" , child , (maybe_init , maybe_uninit)) ; some_maybe_init |= maybe_init ; some_maybe_uninit |= maybe_uninit ; children_count += 1 ; }) ; ((some_maybe_init , some_maybe_uninit) , children_count != 1) } } ; match (maybe_init , maybe_uninit , multipart) { (false , _ , _) => DropStyle :: Dead , (true , false , _) => DropStyle :: Static , (true , true , false) => DropStyle :: Conditional , (true , true , true) => DropStyle :: Open , } } fn clear_drop_flag (& mut self , loc : Location , path : Self :: Path , mode : DropFlagMode) { match mode { DropFlagMode :: Shallow => { self . set_drop_flag (loc , path , DropFlagState :: Absent) ; } DropFlagMode :: Deep => { on_all_children_bits (self . move_data () , path , | child | { self . set_drop_flag (loc , child , DropFlagState :: Absent) }) ; } } } fn field_subpath (& self , path : Self :: Path , field : FieldIdx) -> Option < Self :: Path > { rustc_mir_dataflow :: move_path_children_matching (self . move_data () , path , | e | match e { ProjectionElem :: Field (idx , _) => idx == field , _ => false , }) } fn array_subpath (& self , path : Self :: Path , index : u64 , size : u64) -> Option < Self :: Path > { rustc_mir_dataflow :: move_path_children_matching (self . move_data () , path , | e | match e { ProjectionElem :: ConstantIndex { offset , min_length , from_end } => { debug_assert ! (size == min_length , "min_length should be exact for arrays") ; assert ! (! from_end , "from_end should not be used for array element ConstantIndex") ; offset == index } _ => false , }) } fn deref_subpath (& self , path : Self :: Path) -> Option < Self :: Path > { rustc_mir_dataflow :: move_path_children_matching (self . move_data () , path , | e | { e == ProjectionElem :: Deref }) } fn downcast_subpath (& self , path : Self :: Path , variant : VariantIdx) -> Option < Self :: Path > { rustc_mir_dataflow :: move_path_children_matching (self . move_data () , path , | e | match e { ProjectionElem :: Downcast (_ , idx) => idx == variant , _ => false , }) } fn get_drop_flag (& mut self , path : Self :: Path) -> Option < Operand < 'tcx > > { self . drop_flag (path) . map (Operand :: Copy) } }}}
mkitem!{mkstruct!{struct ElaborateDropsCtxt < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , env : & 'a MoveDataTypingEnv < 'tcx > , init_data : InitializationData < 'a , 'tcx > , drop_flags : IndexVec < MovePathIndex , Option < Local > > , patch : MirPatch < 'tcx > , }}}
mkitem!{mkimpl!{impl fmt :: Debug for ElaborateDropsCtxt < '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ElaborateDropsCtxt") . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ElaborateDropsCtxt < 'a , 'tcx > { fn move_data (& self) -> & 'a MoveData < 'tcx > { & self . env . move_data } fn create_drop_flag (& mut self , index : MovePathIndex , span : Span) { let patch = & mut self . patch ; debug ! ("create_drop_flag({:?})" , self . body . span) ; self . drop_flags [index] . get_or_insert_with (| | patch . new_temp (self . tcx . types . bool , span)) ; } fn drop_flag (& mut self , index : MovePathIndex) -> Option < Place < 'tcx > > { self . drop_flags [index] . map (Place :: from) } #[doc = " create a patch that elaborates all drops in the input"] #[doc = " MIR."] fn elaborate (mut self) -> MirPatch < 'tcx > { self . collect_drop_flags () ; self . elaborate_drops () ; self . drop_flags_on_init () ; self . drop_flags_for_fn_rets () ; self . drop_flags_for_args () ; self . drop_flags_for_locs () ; self . patch } fn collect_drop_flags (& mut self) { for (bb , data) in self . body . basic_blocks . iter_enumerated () { let terminator = data . terminator () ; let TerminatorKind :: Drop { ref place , .. } = terminator . kind else { continue } ; let path = self . move_data () . rev_lookup . find (place . as_ref ()) ; debug ! ("collect_drop_flags: {:?}, place {:?} ({:?})" , bb , place , path) ; match path { LookupResult :: Exact (path) => { self . init_data . seek_before (self . body . terminator_loc (bb)) ; on_all_children_bits (self . move_data () , path , | child | { let (maybe_init , maybe_uninit) = self . init_data . maybe_init_uninit (child) ; debug ! ("collect_drop_flags: collecting {:?} from {:?}@{:?} - {:?}" , child , place , path , (maybe_init , maybe_uninit)) ; if maybe_init && maybe_uninit { self . create_drop_flag (child , terminator . source_info . span) } }) ; } LookupResult :: Parent (None) => { } LookupResult :: Parent (Some (parent)) => { if self . body . local_decls [place . local] . is_deref_temp () { continue ; } self . init_data . seek_before (self . body . terminator_loc (bb)) ; let (_maybe_init , maybe_uninit) = self . init_data . maybe_init_uninit (parent) ; if maybe_uninit { self . tcx . dcx () . span_delayed_bug (terminator . source_info . span , format ! ("drop of untracked, uninitialized value {bb:?}, place {place:?} ({path:?})") ,) ; } } } ; } } fn elaborate_drops (& mut self) { for (bb , data) in self . body . basic_blocks . iter_enumerated () { let terminator = data . terminator () ; let TerminatorKind :: Drop { place , target , unwind , replace , drop , async_fut : _ } = terminator . kind else { continue ; } ; if ! place . ty (& self . body . local_decls , self . tcx) . ty . needs_drop (self . tcx , self . typing_env ()) { self . patch . patch_terminator (bb , TerminatorKind :: Goto { target }) ; continue ; } let path = self . move_data () . rev_lookup . find (place . as_ref ()) ; match path { LookupResult :: Exact (path) => { let unwind = match unwind { _ if data . is_cleanup => Unwind :: InCleanup , UnwindAction :: Cleanup (cleanup) => Unwind :: To (cleanup) , UnwindAction :: Continue => Unwind :: To (self . patch . resume_block ()) , UnwindAction :: Unreachable => { Unwind :: To (self . patch . unreachable_cleanup_block ()) } UnwindAction :: Terminate (reason) => { debug_assert_ne ! (reason , UnwindTerminateReason :: InCleanup , "we are not in a cleanup block, InCleanup reason should be impossible") ; Unwind :: To (self . patch . terminate_block (reason)) } } ; self . init_data . seek_before (self . body . terminator_loc (bb)) ; elaborate_drop (self , terminator . source_info , place , path , target , unwind , bb , drop ,) } LookupResult :: Parent (None) => { } LookupResult :: Parent (Some (_)) => { if ! replace { self . tcx . dcx () . span_bug (terminator . source_info . span , format ! ("drop of untracked value {bb:?}") ,) ; } assert ! (! data . is_cleanup) ; } } } } fn constant_bool (& self , span : Span , val : bool) -> Rvalue < 'tcx > { Rvalue :: Use (Operand :: Constant (Box :: new (ConstOperand { span , user_ty : None , const_ : Const :: from_bool (self . tcx , val) , }))) } fn set_drop_flag (& mut self , loc : Location , path : MovePathIndex , val : DropFlagState) { if let Some (flag) = self . drop_flags [path] { let span = self . patch . source_info_for_location (self . body , loc) . span ; let val = self . constant_bool (span , val . value ()) ; self . patch . add_assign (loc , Place :: from (flag) , val) ; } } fn drop_flags_on_init (& mut self) { let loc = Location :: START ; let span = self . patch . source_info_for_location (self . body , loc) . span ; let false_ = self . constant_bool (span , false) ; for flag in self . drop_flags . iter () . flatten () { self . patch . add_assign (loc , Place :: from (* flag) , false_ . clone ()) ; } } fn drop_flags_for_fn_rets (& mut self) { for (bb , data) in self . body . basic_blocks . iter_enumerated () { if let TerminatorKind :: Call { destination , target : Some (tgt) , unwind : UnwindAction :: Cleanup (_) , .. } = data . terminator () . kind { assert ! (! self . patch . is_term_patched (bb)) ; let loc = Location { block : tgt , statement_index : 0 } ; let path = self . move_data () . rev_lookup . find (destination . as_ref ()) ; on_lookup_result_bits (self . move_data () , path , | child | { self . set_drop_flag (loc , child , DropFlagState :: Present) }) ; } } } fn drop_flags_for_args (& mut self) { let loc = Location :: START ; rustc_mir_dataflow :: drop_flag_effects_for_function_entry (self . body , & self . env . move_data , | path , ds | { self . set_drop_flag (loc , path , ds) ; } ,) } fn drop_flags_for_locs (& mut self) { for (bb , data) in self . body . basic_blocks . iter_enumerated () { debug ! ("drop_flags_for_locs({:?})" , data) ; for i in 0 .. (data . statements . len () + 1) { debug ! ("drop_flag_for_locs: stmt {}" , i) ; if i == data . statements . len () { match data . terminator () . kind { TerminatorKind :: Drop { .. } => { continue ; } TerminatorKind :: UnwindResume => { } _ => { assert ! (! self . patch . is_term_patched (bb)) ; } } } let loc = Location { block : bb , statement_index : i } ; rustc_mir_dataflow :: drop_flag_effects_for_location (self . body , & self . env . move_data , loc , | path , ds | self . set_drop_flag (loc , path , ds) ,) } if let TerminatorKind :: Call { destination , target : Some (_) , unwind : UnwindAction :: Continue | UnwindAction :: Unreachable | UnwindAction :: Terminate (_) , .. } = data . terminator () . kind { assert ! (! self . patch . is_term_patched (bb)) ; let loc = Location { block : bb , statement_index : data . statements . len () } ; let path = self . move_data () . rev_lookup . find (destination . as_ref ()) ; on_lookup_result_bits (self . move_data () , path , | child | { self . set_drop_flag (loc , child , DropFlagState :: Present) }) ; } } } }}}