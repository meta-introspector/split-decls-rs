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
mkuse!{use std :: collections :: HashSet ;}
mkuse!{use std :: env ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use std :: time :: Instant ;}
mkuse!{use gccjit :: { CType , Context , FunctionType , GlobalKind } ;}
mkuse!{use rustc_codegen_ssa :: ModuleCodegen ;}
mkuse!{use rustc_codegen_ssa :: base :: maybe_create_entry_wrapper ;}
mkuse!{use rustc_codegen_ssa :: mono_item :: MonoItemExt ;}
mkuse!{use rustc_codegen_ssa :: traits :: DebugInfoCodegenMethods ;}
mkuse!{use rustc_hir :: attrs :: Linkage ;}
mkuse!{use rustc_middle :: dep_graph ;}
mkuse!{#[cfg (feature = "master")] use rustc_middle :: mir :: mono :: Visibility ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: config :: DebugInfo ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{#[cfg (feature = "master")] use rustc_target :: spec :: SymbolVisibility ;}
mkuse!{use rustc_target :: spec :: { PanicStrategy , RelocModel } ;}
mkuse!{use crate :: builder :: Builder ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: { GccContext , LockedTargetInfo , SyncContext , gcc_util , new_context } ;}

macro_rules! visibility_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visibility_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    visibility_to_gcc_introspect!();
    #[cfg (feature = "master")] pub fn visibility_to_gcc (visibility : Visibility) -> gccjit :: Visibility { match visibility { Visibility :: Default => gccjit :: Visibility :: Default , Visibility :: Hidden => gccjit :: Visibility :: Hidden , Visibility :: Protected => gccjit :: Visibility :: Protected , } }
}

macro_rules! symbol_visibility_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symbol_visibility_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    symbol_visibility_to_gcc_introspect!();
    #[cfg (feature = "master")] pub fn symbol_visibility_to_gcc (visibility : SymbolVisibility) -> gccjit :: Visibility { match visibility { SymbolVisibility :: Hidden => gccjit :: Visibility :: Hidden , SymbolVisibility :: Protected => gccjit :: Visibility :: Protected , SymbolVisibility :: Interposable => gccjit :: Visibility :: Default , } }
}

macro_rules! global_linkage_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function global_linkage_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    global_linkage_to_gcc_introspect!();
    pub fn global_linkage_to_gcc (linkage : Linkage) -> GlobalKind { match linkage { Linkage :: External => GlobalKind :: Imported , Linkage :: AvailableExternally => GlobalKind :: Imported , Linkage :: LinkOnceAny => unimplemented ! () , Linkage :: LinkOnceODR => unimplemented ! () , Linkage :: WeakAny => unimplemented ! () , Linkage :: WeakODR => unimplemented ! () , Linkage :: Internal => GlobalKind :: Internal , Linkage :: ExternalWeak => GlobalKind :: Imported , Linkage :: Common => unimplemented ! () , } }
}

macro_rules! linkage_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function linkage_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    linkage_to_gcc_introspect!();
    pub fn linkage_to_gcc (linkage : Linkage) -> FunctionType { match linkage { Linkage :: External => FunctionType :: Exported , Linkage :: AvailableExternally => FunctionType :: Extern , Linkage :: LinkOnceAny => unimplemented ! () , Linkage :: LinkOnceODR => unimplemented ! () , Linkage :: WeakAny => FunctionType :: Exported , Linkage :: WeakODR => unimplemented ! () , Linkage :: Internal => FunctionType :: Internal , Linkage :: ExternalWeak => unimplemented ! () , Linkage :: Common => unimplemented ! () , } }
}

macro_rules! compile_codegen_unit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compile_codegen_unit in module {}", module_path!());
    };
}

mkfn!{
    compile_codegen_unit_introspect!();
    pub fn compile_codegen_unit (tcx : TyCtxt < '_ > , cgu_name : Symbol , target_info : LockedTargetInfo ,) -> (ModuleCodegen < GccContext > , u64) { let prof_timer = tcx . prof . generic_activity ("codegen_module") ; let start_time = Instant :: now () ; let dep_node = tcx . codegen_unit (cgu_name) . codegen_dep_node (tcx) ; let (module , _) = tcx . dep_graph . with_task (dep_node , tcx , (cgu_name , target_info) , module_codegen , Some (dep_graph :: hash_result) ,) ; let time_to_codegen = start_time . elapsed () ; drop (prof_timer) ; let cost = time_to_codegen . as_secs () * 1_000_000_000 + time_to_codegen . subsec_nanos () as u64 ; fn module_codegen (tcx : TyCtxt < '_ > , (cgu_name , target_info) : (Symbol , LockedTargetInfo) ,) -> ModuleCodegen < GccContext > { let cgu = tcx . codegen_unit (cgu_name) ; let context = new_context (tcx) ; if tcx . sess . panic_strategy () == PanicStrategy :: Unwind { context . add_command_line_option ("-fexceptions") ; context . add_driver_option ("-fexceptions") ; } let disabled_features : HashSet < _ > = tcx . sess . opts . cg . target_feature . split (',') . filter (| feature | feature . starts_with ('-')) . map (| string | & string [1 ..]) . collect () ; if ! disabled_features . contains ("avx") && tcx . sess . target . arch == "x86_64" { context . add_command_line_option ("-mavx") ; } for arg in & tcx . sess . opts . cg . llvm_args { context . add_command_line_option (arg) ; } context . add_command_line_option ("-fno-var-tracking-assignments") ; context . add_command_line_option ("-fno-semantic-interposition") ; context . add_command_line_option ("-fno-strict-aliasing") ; context . add_command_line_option ("-fwrapv") ; if let Some (model) = tcx . sess . code_model () { use rustc_target :: spec :: CodeModel ; context . add_command_line_option (match model { CodeModel :: Tiny => "-mcmodel=tiny" , CodeModel :: Small => "-mcmodel=small" , CodeModel :: Kernel => "-mcmodel=kernel" , CodeModel :: Medium => "-mcmodel=medium" , CodeModel :: Large => "-mcmodel=large" , }) ; } add_pic_option (& context , tcx . sess . relocation_model ()) ; let target_cpu = gcc_util :: target_cpu (tcx . sess) ; if target_cpu != "generic" { context . add_command_line_option (format ! ("-march={}" , target_cpu)) ; } if tcx . sess . opts . unstable_opts . function_sections . unwrap_or (tcx . sess . target . function_sections) { context . add_command_line_option ("-ffunction-sections") ; context . add_command_line_option ("-fdata-sections") ; } if env :: var ("CG_GCCJIT_DUMP_RTL") . as_deref () == Ok ("1") { context . add_command_line_option ("-fdump-rtl-vregs") ; } if env :: var ("CG_GCCJIT_DUMP_RTL_ALL") . as_deref () == Ok ("1") { context . add_command_line_option ("-fdump-rtl-all") ; } if env :: var ("CG_GCCJIT_DUMP_TREE_ALL") . as_deref () == Ok ("1") { context . add_command_line_option ("-fdump-tree-all-eh") ; } if env :: var ("CG_GCCJIT_DUMP_IPA_ALL") . as_deref () == Ok ("1") { context . add_command_line_option ("-fdump-ipa-all-eh") ; } if env :: var ("CG_GCCJIT_DUMP_CODE") . as_deref () == Ok ("1") { context . set_dump_code_on_compile (true) ; } if env :: var ("CG_GCCJIT_DUMP_GIMPLE") . as_deref () == Ok ("1") { context . set_dump_initial_gimple (true) ; } if env :: var ("CG_GCCJIT_DUMP_EVERYTHING") . as_deref () == Ok ("1") { context . set_dump_everything (true) ; } if env :: var ("CG_GCCJIT_KEEP_INTERMEDIATES") . as_deref () == Ok ("1") { context . set_keep_intermediates (true) ; } if env :: var ("CG_GCCJIT_VERBOSE") . as_deref () == Ok ("1") { context . add_driver_option ("-v") ; } context . set_allow_unreachable_blocks (true) ; { let f16_type_supported = target_info . supports_target_dependent_type (CType :: Float16) ; let f32_type_supported = target_info . supports_target_dependent_type (CType :: Float32) ; let f64_type_supported = target_info . supports_target_dependent_type (CType :: Float64) ; let f128_type_supported = target_info . supports_target_dependent_type (CType :: Float128) ; let u128_type_supported = target_info . supports_target_dependent_type (CType :: UInt128t) ; let mut cx = CodegenCx :: new (& context , cgu , tcx , u128_type_supported , f16_type_supported , f32_type_supported , f64_type_supported , f128_type_supported ,) ; let mono_items = cgu . items_in_deterministic_order (tcx) ; for & (mono_item , data) in & mono_items { mono_item . predefine :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , data . linkage , data . visibility ,) ; } for & (mono_item , item_data) in & mono_items { mono_item . define :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , item_data) ; } maybe_create_entry_wrapper :: < Builder < '_ , '_ , '_ > > (& cx , cx . codegen_unit) ; if cx . sess () . opts . debuginfo != DebugInfo :: None { cx . debuginfo_finalize () ; } } ModuleCodegen :: new_regular (cgu_name . to_string () , GccContext { context : Arc :: new (SyncContext :: new (context)) , relocation_model : tcx . sess . relocation_model () , should_combine_object_files : false , temp_dir : None , } ,) } (module , cost) }
}

macro_rules! add_pic_option_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_pic_option in module {}", module_path!());
    };
}

mkfn!{
    add_pic_option_introspect!();
    pub fn add_pic_option < 'gcc > (context : & Context < 'gcc > , relocation_model : RelocModel) { match relocation_model { rustc_target :: spec :: RelocModel :: Static => { context . add_command_line_option ("-fno-pie") ; context . add_driver_option ("-fno-pie") ; } rustc_target :: spec :: RelocModel :: Pic => { context . add_command_line_option ("-fPIC") ; context . add_driver_option ("-fPIC") ; } rustc_target :: spec :: RelocModel :: Pie => { context . add_command_line_option ("-fPIE") ; context . add_driver_option ("-fPIE") ; } model => eprintln ! ("Unsupported relocation model: {:?}" , model) , } }
}