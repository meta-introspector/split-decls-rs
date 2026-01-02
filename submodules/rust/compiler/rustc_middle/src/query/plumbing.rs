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
mkuse!{use std :: ops :: Deref ;}
mkuse!{use rustc_data_structures :: sync :: { AtomicU64 , WorkerLocal } ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: hir_id :: OwnerId ;}
mkuse!{use rustc_macros :: HashStable ;}
mkuse!{use rustc_query_system :: HandleCycleError ;}
mkuse!{use rustc_query_system :: dep_graph :: { DepNodeIndex , SerializedDepNodeIndex } ;}
mkuse!{pub (crate) use rustc_query_system :: query :: QueryJobId ;}
mkuse!{use rustc_query_system :: query :: * ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed , Span } ;}
mkuse!{use crate :: dep_graph ;}
mkuse!{use crate :: dep_graph :: DepKind ;}
mkuse!{use crate :: query :: on_disk_cache :: { CacheEncoder , EncodedDepNodeIndex , OnDiskCache } ;}
mkuse!{use crate :: query :: { DynamicQueries , ExternProviders , Providers , QueryArenas , QueryCaches , QueryEngine , QueryStates , } ;}
mkuse!{use crate :: ty :: TyCtxt ;}
mkitem!{mkstruct!{pub struct DynamicQuery < 'tcx , C : QueryCache > { pub name : & 'static str , pub eval_always : bool , pub dep_kind : DepKind , pub handle_cycle_error : HandleCycleError , pub query_state : usize , pub query_cache : usize , pub cache_on_disk : fn (tcx : TyCtxt < 'tcx > , key : & C :: Key) -> bool , pub execute_query : fn (tcx : TyCtxt < 'tcx > , k : C :: Key) -> C :: Value , pub compute : fn (tcx : TyCtxt < 'tcx > , key : C :: Key) -> C :: Value , pub can_load_from_disk : bool , pub try_load_from_disk : fn (tcx : TyCtxt < 'tcx > , key : & C :: Key , prev_index : SerializedDepNodeIndex , index : DepNodeIndex ,) -> Option < C :: Value > , pub loadable_from_disk : fn (tcx : TyCtxt < 'tcx > , key : & C :: Key , index : SerializedDepNodeIndex) -> bool , pub hash_result : HashResult < C :: Value > , pub value_from_cycle_error : fn (tcx : TyCtxt < 'tcx > , cycle_error : & CycleError , guar : ErrorGuaranteed) -> C :: Value , pub format_value : fn (& C :: Value) -> String , }}}
mkitem!{mkstruct!{pub struct QuerySystemFns { pub engine : QueryEngine , pub local_providers : Providers , pub extern_providers : ExternProviders , pub encode_query_results : for < 'tcx > fn (tcx : TyCtxt < 'tcx > , encoder : & mut CacheEncoder < '_ , 'tcx > , query_result_index : & mut EncodedDepNodeIndex ,) , pub try_mark_green : for < 'tcx > fn (tcx : TyCtxt < 'tcx > , dep_node : & dep_graph :: DepNode) -> bool , }}}
mkitem!{mkstruct!{pub struct QuerySystem < 'tcx > { pub states : QueryStates < 'tcx > , pub arenas : WorkerLocal < QueryArenas < 'tcx > > , pub caches : QueryCaches < 'tcx > , pub dynamic_queries : DynamicQueries < 'tcx > , #[doc = " This provides access to the incremental compilation on-disk cache for query results."] #[doc = " Do not access this directly. It is only meant to be used by"] #[doc = " `DepGraph::try_mark_green()` and the query infrastructure."] #[doc = " This is `None` if we are not incremental compilation mode"] pub on_disk_cache : Option < OnDiskCache > , pub fns : QuerySystemFns , pub jobs : AtomicU64 , }}}
mkitem!{mkstruct!{#[derive (Copy , Clone)] pub struct TyCtxtAt < 'tcx > { pub tcx : TyCtxt < 'tcx > , pub span : Span , }}}
mkitem!{mkimpl!{impl < 'tcx > Deref for TyCtxtAt < 'tcx > { type Target = TyCtxt < 'tcx > ; #[inline (always)] fn deref (& self) -> & Self :: Target { & self . tcx } }}}
mkitem!{mkstruct!{#[derive (Copy , Clone)] #[must_use] pub struct TyCtxtEnsureOk < 'tcx > { pub tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkstruct!{#[derive (Copy , Clone)] #[must_use] pub struct TyCtxtEnsureDone < 'tcx > { pub tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > TyCtxt < 'tcx > { #[doc = " Wrapper that calls queries in a special \"ensure OK\" mode, for callers"] #[doc = " that don't need the return value and just want to invoke a query for"] #[doc = " its potential side-effect of emitting fatal errors."] #[doc = ""] #[doc = " This can be more efficient than a normal query call, because if the"] #[doc = " query's inputs are all green, the call can return immediately without"] #[doc = " needing to obtain a value (by decoding one from disk or by executing"] #[doc = " the query)."] #[doc = ""] #[doc = " (As with all query calls, execution is also skipped if the query result"] #[doc = " is already cached in memory.)"] #[doc = ""] #[doc = " ## WARNING"] #[doc = " A subsequent normal call to the same query might still cause it to be"] #[doc = " executed! This can occur when the inputs are all green, but the query's"] #[doc = " result is not cached on disk, so the query must be executed to obtain a"] #[doc = " return value."] #[doc = ""] #[doc = " Therefore, this call mode is not appropriate for callers that want to"] #[doc = " ensure that the query is _never_ executed in the future."] #[doc = ""] #[doc = " ## `return_result_from_ensure_ok`"] #[doc = " If a query has the `return_result_from_ensure_ok` modifier, calls via"] #[doc = " `ensure_ok` will instead return `Result<(), ErrorGuaranteed>`. If the"] #[doc = " query needs to be executed, and execution returns an error, that error"] #[doc = " is returned to the caller."] #[inline (always)] pub fn ensure_ok (self) -> TyCtxtEnsureOk < 'tcx > { TyCtxtEnsureOk { tcx : self } } #[doc = " Wrapper that calls queries in a special \"ensure done\" mode, for callers"] #[doc = " that don't need the return value and just want to guarantee that the"] #[doc = " query won't be executed in the future, by executing it now if necessary."] #[doc = ""] #[doc = " This is useful for queries that read from a [`Steal`] value, to ensure"] #[doc = " that they are executed before the query that will steal the value."] #[doc = ""] #[doc = " Unlike [`Self::ensure_ok`], a query with all-green inputs will only be"] #[doc = " skipped if its return value is stored in the disk-cache. This is still"] #[doc = " more efficient than a regular query, because in that situation the"] #[doc = " return value doesn't necessarily need to be decoded."] #[doc = ""] #[doc = " (As with all query calls, execution is also skipped if the query result"] #[doc = " is already cached in memory.)"] #[doc = ""] #[doc = " [`Steal`]: rustc_data_structures::steal::Steal"] #[inline (always)] pub fn ensure_done (self) -> TyCtxtEnsureDone < 'tcx > { TyCtxtEnsureDone { tcx : self } } #[doc = " Returns a transparent wrapper for `TyCtxt` which uses"] #[doc = " `span` as the location of queries performed through it."] #[inline (always)] pub fn at (self , span : Span) -> TyCtxtAt < 'tcx > { TyCtxtAt { tcx : self , span } } pub fn try_mark_green (self , dep_node : & dep_graph :: DepNode) -> bool { (self . query_system . fns . try_mark_green) (self , dep_node) } }}}

macro_rules! query_get_at_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function query_get_at in module {}", module_path!());
    };
}

mkfn!{
    query_get_at_introspect!();
    #[inline (always)] pub fn query_get_at < 'tcx , Cache > (tcx : TyCtxt < 'tcx > , execute_query : fn (TyCtxt < 'tcx > , Span , Cache :: Key , QueryMode) -> Option < Cache :: Value > , query_cache : & Cache , span : Span , key : Cache :: Key ,) -> Cache :: Value where Cache : QueryCache , { let key = key . into_query_param () ; match try_get_cached (tcx , query_cache , & key) { Some (value) => value , None => execute_query (tcx , span , key , QueryMode :: Get) . unwrap () , } }
}

macro_rules! query_ensure_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function query_ensure in module {}", module_path!());
    };
}

mkfn!{
    query_ensure_introspect!();
    #[inline] pub fn query_ensure < 'tcx , Cache > (tcx : TyCtxt < 'tcx > , execute_query : fn (TyCtxt < 'tcx > , Span , Cache :: Key , QueryMode) -> Option < Cache :: Value > , query_cache : & Cache , key : Cache :: Key , check_cache : bool ,) where Cache : QueryCache , { let key = key . into_query_param () ; if try_get_cached (tcx , query_cache , & key) . is_none () { execute_query (tcx , DUMMY_SP , key , QueryMode :: Ensure { check_cache }) ; } }
}

macro_rules! query_ensure_error_guaranteed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function query_ensure_error_guaranteed in module {}", module_path!());
    };
}

mkfn!{
    query_ensure_error_guaranteed_introspect!();
    #[inline] pub fn query_ensure_error_guaranteed < 'tcx , Cache , T > (tcx : TyCtxt < 'tcx > , execute_query : fn (TyCtxt < 'tcx > , Span , Cache :: Key , QueryMode) -> Option < Cache :: Value > , query_cache : & Cache , key : Cache :: Key , check_cache : bool ,) -> Result < () , ErrorGuaranteed > where Cache : QueryCache < Value = super :: erase :: Erase < Result < T , ErrorGuaranteed > > > , Result < T , ErrorGuaranteed > : EraseType , { let key = key . into_query_param () ; if let Some (res) = try_get_cached (tcx , query_cache , & key) { super :: erase :: restore (res) . map (drop) } else { execute_query (tcx , DUMMY_SP , key , QueryMode :: Ensure { check_cache }) . map (super :: erase :: restore) . map (| res | res . map (drop)) . unwrap_or (Ok (())) } }
}
mkitem!{macro_rules ! query_ensure { ([] $ ($ args : tt) *) => { query_ensure ($ ($ args) *) } ; ([(return_result_from_ensure_ok) $ ($ rest : tt) *] $ ($ args : tt) *) => { query_ensure_error_guaranteed ($ ($ args) *) . map (| _ | ()) } ; ([$ other : tt $ ($ modifiers : tt) *] $ ($ args : tt) *) => { query_ensure ! ([$ ($ modifiers) *] $ ($ args) *) } ; }}
mkitem!{macro_rules ! query_helper_param_ty { (DefId) => { impl IntoQueryParam < DefId > } ; (LocalDefId) => { impl IntoQueryParam < LocalDefId > } ; ($ K : ty) => { $ K } ; }}
mkitem!{macro_rules ! query_if_arena { ([] $ arena : tt $ no_arena : tt) => { $ no_arena } ; ([(arena_cache) $ ($ rest : tt) *] $ arena : tt $ no_arena : tt) => { $ arena } ; ([$ other : tt $ ($ modifiers : tt) *] $ ($ args : tt) *) => { query_if_arena ! ([$ ($ modifiers) *] $ ($ args) *) } ; }}
mkitem!{#[doc = " If `separate_provide_extern`, then the key can be projected to its"] #[doc = " local key via `<$K as AsLocalKey>::LocalKey`."] macro_rules ! local_key_if_separate_extern { ([] $ ($ K : tt) *) => { $ ($ K) * } ; ([(separate_provide_extern) $ ($ rest : tt) *] $ ($ K : tt) *) => { <$ ($ K) * as AsLocalKey >:: LocalKey } ; ([$ other : tt $ ($ modifiers : tt) *] $ ($ K : tt) *) => { local_key_if_separate_extern ! ([$ ($ modifiers) *] $ ($ K) *) } ; }}
mkitem!{macro_rules ! separate_provide_extern_decl { ([] [$ name : ident]) => { () } ; ([(separate_provide_extern) $ ($ rest : tt) *] [$ name : ident]) => { for <'tcx > fn (TyCtxt <'tcx >, queries ::$ name :: Key <'tcx >,) -> queries ::$ name :: ProvidedValue <'tcx > } ; ([$ other : tt $ ($ modifiers : tt) *] [$ ($ args : tt) *]) => { separate_provide_extern_decl ! ([$ ($ modifiers) *] [$ ($ args) *]) } ; }}
mkitem!{macro_rules ! ensure_ok_result { ([]) => { () } ; ([(return_result_from_ensure_ok) $ ($ rest : tt) *]) => { Result < () , ErrorGuaranteed > } ; ([$ other : tt $ ($ modifiers : tt) *]) => { ensure_ok_result ! ([$ ($ modifiers) *]) } ; }}
mkitem!{macro_rules ! separate_provide_extern_default { ([] [$ name : ident]) => { () } ; ([(separate_provide_extern) $ ($ rest : tt) *] [$ name : ident]) => { | _ , key | $ crate :: query :: plumbing :: default_extern_query (stringify ! ($ name) , & key) } ; ([$ other : tt $ ($ modifiers : tt) *] [$ ($ args : tt) *]) => { separate_provide_extern_default ! ([$ ($ modifiers) *] [$ ($ args) *]) } ; }}
mkitem!{macro_rules ! define_callbacks { ($ ($ (#[$ attr : meta]) * [$ ($ modifiers : tt) *] fn $ name : ident ($ ($ K : tt) *) -> $ V : ty ,) *) => { #[allow (unused_lifetimes)] pub mod queries { $ (pub mod $ name { use super :: super ::*; pub type Key <'tcx > = $ ($ K) *; pub type Value <'tcx > = $ V ; pub type LocalKey <'tcx > = local_key_if_separate_extern ! ([$ ($ modifiers) *] $ ($ K) *) ; #[doc = " This type alias specifies the type returned from query providers and the type"] #[doc = " used for decoding. For regular queries this is the declared returned type `V`,"] #[doc = " but `arena_cache` will use `<V as ArenaCached>::Provided` instead."] pub type ProvidedValue <'tcx > = query_if_arena ! ([$ ($ modifiers) *] (<$ V as $ crate :: query :: arena_cached :: ArenaCached <'tcx >>:: Provided) ($ V)) ; #[doc = " This function takes `ProvidedValue` and coverts it to an erased `Value` by"] #[doc = " allocating it on an arena if the query has the `arena_cache` modifier. The"] #[doc = " value is then erased and returned. This will happen when computing the query"] #[doc = " using a provider or decoding a stored result."] #[inline (always)] pub fn provided_to_erased <'tcx > (_tcx : TyCtxt <'tcx >, value : ProvidedValue <'tcx >,) -> Erase < Value <'tcx >> { erase (query_if_arena ! ([$ ($ modifiers) *] { use $ crate :: query :: arena_cached :: ArenaCached ; if mem :: needs_drop ::<<$ V as ArenaCached <'tcx >>:: Allocated > () { <$ V as ArenaCached >:: alloc_in_arena (| v | _tcx . query_system . arenas .$ name . alloc (v) , value ,) } else { <$ V as ArenaCached >:: alloc_in_arena (| v | _tcx . arena . dropless . alloc (v) , value ,) } } (value))) } pub type Storage <'tcx > = <$ ($ K) * as keys :: Key >:: Cache < Erase <$ V >>; #[cfg (target_pointer_width = "64")] const _ : () = { if size_of ::< Key <'static >> () > 88 { panic ! ("{}" , concat ! ("the query `" , stringify ! ($ name) , "` has a key type `" , stringify ! ($ ($ K) *) , "` that is too large")) ; } } ; #[cfg (target_pointer_width = "64")] #[cfg (not (feature = "rustc_randomized_layouts"))] const _ : () = { if size_of ::< Value <'static >> () > 64 { panic ! ("{}" , concat ! ("the query `" , stringify ! ($ name) , "` has a value type `" , stringify ! ($ V) , "` that is too large")) ; } } ; }) * } pub struct QueryArenas <'tcx > { $ ($ (#[$ attr]) * pub $ name : query_if_arena ! ([$ ($ modifiers) *] (TypedArena <<$ V as $ crate :: query :: arena_cached :: ArenaCached <'tcx >>:: Allocated >) ()) ,) * } impl Default for QueryArenas <'_ > { fn default () -> Self { Self { $ ($ name : query_if_arena ! ([$ ($ modifiers) *] (Default :: default ()) ()) ,) * } } } #[derive (Default)] pub struct QueryCaches <'tcx > { $ ($ (#[$ attr]) * pub $ name : queries ::$ name :: Storage <'tcx >,) * } impl <'tcx > TyCtxtEnsureOk <'tcx > { $ ($ (#[$ attr]) * #[inline (always)] pub fn $ name (self , key : query_helper_param_ty ! ($ ($ K) *) ,) -> ensure_ok_result ! ([$ ($ modifiers) *]) { query_ensure ! ([$ ($ modifiers) *] self . tcx , self . tcx . query_system . fns . engine .$ name , & self . tcx . query_system . caches .$ name , key . into_query_param () , false ,) }) * } impl <'tcx > TyCtxtEnsureDone <'tcx > { $ ($ (#[$ attr]) * #[inline (always)] pub fn $ name (self , key : query_helper_param_ty ! ($ ($ K) *)) { query_ensure (self . tcx , self . tcx . query_system . fns . engine .$ name , & self . tcx . query_system . caches .$ name , key . into_query_param () , true ,) ; }) * } impl <'tcx > TyCtxt <'tcx > { $ ($ (#[$ attr]) * #[inline (always)] #[must_use] pub fn $ name (self , key : query_helper_param_ty ! ($ ($ K) *)) -> $ V { self . at (DUMMY_SP) .$ name (key) }) * } impl <'tcx > TyCtxtAt <'tcx > { $ ($ (#[$ attr]) * #[inline (always)] pub fn $ name (self , key : query_helper_param_ty ! ($ ($ K) *)) -> $ V { restore ::<$ V > (query_get_at (self . tcx , self . tcx . query_system . fns . engine .$ name , & self . tcx . query_system . caches .$ name , self . span , key . into_query_param () ,)) }) * } pub struct DynamicQueries <'tcx > { $ (pub $ name : DynamicQuery <'tcx , queries ::$ name :: Storage <'tcx >>,) * } #[derive (Default)] pub struct QueryStates <'tcx > { $ (pub $ name : QueryState <$ ($ K) *, QueryStackDeferred <'tcx >>,) * } pub struct Providers { $ (pub $ name : for <'tcx > fn (TyCtxt <'tcx >, queries ::$ name :: LocalKey <'tcx >,) -> queries ::$ name :: ProvidedValue <'tcx >,) * } pub struct ExternProviders { $ (pub $ name : separate_provide_extern_decl ! ([$ ($ modifiers) *] [$ name]) ,) * } impl Default for Providers { fn default () -> Self { Providers { $ ($ name : | _ , key | $ crate :: query :: plumbing :: default_query (stringify ! ($ name) , & key)) ,* } } } impl Default for ExternProviders { fn default () -> Self { ExternProviders { $ ($ name : separate_provide_extern_default ! ([$ ($ modifiers) *] [$ name]) ,) * } } } impl Copy for Providers { } impl Clone for Providers { fn clone (& self) -> Self { * self } } impl Copy for ExternProviders { } impl Clone for ExternProviders { fn clone (& self) -> Self { * self } } pub struct QueryEngine { $ (pub $ name : for <'tcx > fn (TyCtxt <'tcx >, Span , queries ::$ name :: Key <'tcx >, QueryMode ,) -> Option < Erase <$ V >>,) * } } ; }}
mkitem!{macro_rules ! hash_result { ([]) => { { Some (dep_graph :: hash_result) } } ; ([(no_hash) $ ($ rest : tt) *]) => { { None } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { hash_result ! ([$ ($ modifiers) *]) } ; }}
mkitem!{macro_rules ! define_feedable { ($ ($ (#[$ attr : meta]) * [$ ($ modifiers : tt) *] fn $ name : ident ($ ($ K : tt) *) -> $ V : ty ,) *) => { $ (impl <'tcx , K : IntoQueryParam <$ ($ K) *> + Copy > TyCtxtFeed <'tcx , K > { $ (#[$ attr]) * #[inline (always)] pub fn $ name (self , value : queries ::$ name :: ProvidedValue <'tcx >) { let key = self . key () . into_query_param () ; let tcx = self . tcx ; let erased = queries ::$ name :: provided_to_erased (tcx , value) ; let value = restore ::<$ V > (erased) ; let cache = & tcx . query_system . caches .$ name ; let hasher : Option < fn (& mut StableHashingContext <'_ >, & _) -> _ > = hash_result ! ([$ ($ modifiers) *]) ; match try_get_cached (tcx , cache , & key) { Some (old) => { let old = restore ::<$ V > (old) ; if let Some (hasher) = hasher { let (value_hash , old_hash) : (Fingerprint , Fingerprint) = tcx . with_stable_hashing_context (| mut hcx | (hasher (& mut hcx , & value) , hasher (& mut hcx , & old))) ; if old_hash != value_hash { tcx . dcx () . delayed_bug (format ! ("Trying to feed an already recorded value for query {} key={key:?}:\n\
                                    old value: {old:?}\nnew value: {value:?}" , stringify ! ($ name) ,)) ; } } else { bug ! ("Trying to feed an already recorded value for query {} key={key:?}:\nold value: {old:?}\nnew value: {value:?}" , stringify ! ($ name) ,) } } None => { let dep_node = dep_graph :: DepNode :: construct (tcx , dep_graph :: dep_kinds ::$ name , & key) ; let dep_node_index = tcx . dep_graph . with_feed_task (dep_node , tcx , & value , hash_result ! ([$ ($ modifiers) *]) ,) ; cache . complete (key , erased , dep_node_index) ; } } } }) * } }}
mkmod!{sealed, { 
                getname!(sealed);
                getsrc!(sealed);
                getpath!(sealed);
                get_deps!(sealed);
                get_crates!(sealed);
                mkinclude!(sealed);
                mkuse!{use rustc_hir :: def_id :: { LocalModDefId , ModDefId } ;}
mkuse!{use super :: { DefId , LocalDefId , OwnerId } ;}
mkitem!{mktrait!{#[doc = " An analogue of the `Into` trait that's intended only for query parameters."] #[doc = ""] #[doc = " This exists to allow queries to accept either `DefId` or `LocalDefId` while requiring that the"] #[doc = " user call `to_def_id` to convert between them everywhere else."] pub trait IntoQueryParam < P > { fn into_query_param (self) -> P ; }}}
mkitem!{mkimpl!{impl < P > IntoQueryParam < P > for P { #[inline (always)] fn into_query_param (self) -> P { self } }}}
mkitem!{mkimpl!{impl < 'a , P : Copy > IntoQueryParam < P > for & 'a P { #[inline (always)] fn into_query_param (self) -> P { * self } }}}
mkitem!{mkimpl!{impl IntoQueryParam < LocalDefId > for OwnerId { #[inline (always)] fn into_query_param (self) -> LocalDefId { self . def_id } }}}
mkitem!{mkimpl!{impl IntoQueryParam < DefId > for LocalDefId { #[inline (always)] fn into_query_param (self) -> DefId { self . to_def_id () } }}}
mkitem!{mkimpl!{impl IntoQueryParam < DefId > for OwnerId { #[inline (always)] fn into_query_param (self) -> DefId { self . to_def_id () } }}}
mkitem!{mkimpl!{impl IntoQueryParam < DefId > for ModDefId { #[inline (always)] fn into_query_param (self) -> DefId { self . to_def_id () } }}}
mkitem!{mkimpl!{impl IntoQueryParam < DefId > for LocalModDefId { #[inline (always)] fn into_query_param (self) -> DefId { self . to_def_id () } }}}
mkitem!{mkimpl!{impl IntoQueryParam < LocalDefId > for LocalModDefId { #[inline (always)] fn into_query_param (self) -> LocalDefId { self . into () } }}} 
            }}
mkuse!{pub use sealed :: IntoQueryParam ;}
mkuse!{use super :: erase :: EraseType ;}
mkitem!{mkstruct!{#[derive (Copy , Clone , Debug , HashStable)] pub struct CyclePlaceholder (pub ErrorGuaranteed) ;}}

macro_rules! default_query_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_query in module {}", module_path!());
    };
}

mkfn!{
    default_query_introspect!();
    #[cold] pub (crate) fn default_query (name : & str , key : & dyn std :: fmt :: Debug) -> ! { bug ! ("`tcx.{name}({key:?})` is not supported for this key;\n\
        hint: Queries can be either made to the local crate, or the external crate. \
        This error means you tried to use it for one that's not supported.\n\
        If that's not the case, {name} was likely never assigned to a provider function.\n" ,) }
}

macro_rules! default_extern_query_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_extern_query in module {}", module_path!());
    };
}

mkfn!{
    default_extern_query_introspect!();
    #[cold] pub (crate) fn default_extern_query (name : & str , key : & dyn std :: fmt :: Debug) -> ! { bug ! ("`tcx.{name}({key:?})` unsupported by its crate; \
         perhaps the `{name}` query was never assigned a provider function" ,) }
}