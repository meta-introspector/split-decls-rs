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
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (# [$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (# [$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{# [macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{# [macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { compile_error ! (concat ! ("USE|" , module_path ! () , "|" , stringify ! ($ use_stmt))) ; } ; }}
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
mkuse!{use std :: collections :: hash_map :: Entry :: * ;}
mkuse!{use rustc_abi :: { CanonAbi , X86Call } ;}
mkuse!{use rustc_ast :: expand :: allocator :: { ALLOCATOR_METHODS , NO_ALLOC_SHIM_IS_UNSTABLE , global_fn_name } ;}
mkuse!{use rustc_data_structures :: unord :: UnordMap ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId , DefIdMap , LOCAL_CRATE , LocalDefId } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{use rustc_middle :: middle :: exported_symbols :: { ExportedSymbol , SymbolExportInfo , SymbolExportKind , SymbolExportLevel , } ;}
mkuse!{use rustc_middle :: query :: LocalCrate ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgKind , GenericArgsRef , Instance , SymbolName , Ty , TyCtxt } ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_session :: config :: { CrateType , OomStrategy } ;}
mkuse!{use rustc_symbol_mangling :: mangle_internal_symbol ;}
mkuse!{use rustc_target :: spec :: TlsModel ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: back :: symbol_export ;}

macro_rules! threshold_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function threshold in module {}", module_path!());
    };
}

mkfn!{
    threshold_introspect!();
    fn threshold (tcx : TyCtxt < '_ >) -> SymbolExportLevel { crates_export_threshold (tcx . crate_types ()) }
}

macro_rules! crate_export_threshold_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crate_export_threshold in module {}", module_path!());
    };
}

mkfn!{
    crate_export_threshold_introspect!();
    fn crate_export_threshold (crate_type : CrateType) -> SymbolExportLevel { match crate_type { CrateType :: Executable | CrateType :: Staticlib | CrateType :: ProcMacro | CrateType :: Cdylib => { SymbolExportLevel :: C } CrateType :: Rlib | CrateType :: Dylib | CrateType :: Sdylib => SymbolExportLevel :: Rust , } }
}

macro_rules! crates_export_threshold_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crates_export_threshold in module {}", module_path!());
    };
}

mkfn!{
    crates_export_threshold_introspect!();
    pub fn crates_export_threshold (crate_types : & [CrateType]) -> SymbolExportLevel { if crate_types . iter () . any (| & crate_type | crate_export_threshold (crate_type) == SymbolExportLevel :: Rust) { SymbolExportLevel :: Rust } else { SymbolExportLevel :: C } }
}

macro_rules! reachable_non_generics_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reachable_non_generics_provider in module {}", module_path!());
    };
}

mkfn!{
    reachable_non_generics_provider_introspect!();
    fn reachable_non_generics_provider (tcx : TyCtxt < '_ > , _ : LocalCrate) -> DefIdMap < SymbolExportInfo > { if ! tcx . sess . opts . output_types . should_codegen () && ! tcx . is_sdylib_interface_build () { return Default :: default () ; } let special_runtime_crate = tcx . is_panic_runtime (LOCAL_CRATE) || tcx . is_compiler_builtins (LOCAL_CRATE) ; let mut reachable_non_generics : DefIdMap < _ > = tcx . reachable_set (()) . items () . filter_map (| & def_id | { if let Some (parent_id) = tcx . opt_local_parent (def_id) && let DefKind :: ForeignMod = tcx . def_kind (parent_id) { let library = tcx . native_library (def_id) ? ; return library . kind . is_statically_included () . then_some (def_id) ; } match tcx . def_kind (def_id) { DefKind :: Fn | DefKind :: Static { .. } => { } DefKind :: AssocFn if tcx . impl_of_assoc (def_id . to_def_id ()) . is_some () => { } _ => return None , } ; let generics = tcx . generics_of (def_id) ; if generics . requires_monomorphization (tcx) { return None ; } if Instance :: mono (tcx , def_id . into ()) . def . requires_inline (tcx) { return None ; } if tcx . cross_crate_inlinable (def_id) { None } else { Some (def_id) } }) . map (| def_id | { let name = tcx . symbol_name (Instance :: mono (tcx , def_id . to_def_id ())) . name ; let used = name == "rust_eh_personality" ; let export_level = if special_runtime_crate { SymbolExportLevel :: Rust } else { symbol_export_level (tcx , def_id . to_def_id ()) } ; let codegen_attrs = tcx . codegen_fn_attrs (def_id . to_def_id ()) ; debug ! ("EXPORTED SYMBOL (local): {} ({:?})" , tcx . symbol_name (Instance :: mono (tcx , def_id . to_def_id ())) , export_level) ; let info = SymbolExportInfo { level : export_level , kind : if tcx . is_static (def_id . to_def_id ()) { if codegen_attrs . flags . contains (CodegenFnAttrFlags :: THREAD_LOCAL) { SymbolExportKind :: Tls } else { SymbolExportKind :: Data } } else { SymbolExportKind :: Text } , used : codegen_attrs . flags . contains (CodegenFnAttrFlags :: USED_COMPILER) || codegen_attrs . flags . contains (CodegenFnAttrFlags :: USED_LINKER) || used , rustc_std_internal_symbol : codegen_attrs . flags . contains (CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL) , } ; (def_id . to_def_id () , info) }) . into () ; if let Some (id) = tcx . proc_macro_decls_static (()) { reachable_non_generics . insert (id . to_def_id () , SymbolExportInfo { level : SymbolExportLevel :: C , kind : SymbolExportKind :: Data , used : false , rustc_std_internal_symbol : false , } ,) ; } reachable_non_generics }
}

macro_rules! is_reachable_non_generic_provider_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_reachable_non_generic_provider_local in module {}", module_path!());
    };
}

mkfn!{
    is_reachable_non_generic_provider_local_introspect!();
    fn is_reachable_non_generic_provider_local (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { let export_threshold = threshold (tcx) ; if let Some (& info) = tcx . reachable_non_generics (LOCAL_CRATE) . get (& def_id . to_def_id ()) { info . level . is_below_threshold (export_threshold) } else { false } }
}

macro_rules! is_reachable_non_generic_provider_extern_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_reachable_non_generic_provider_extern in module {}", module_path!());
    };
}

mkfn!{
    is_reachable_non_generic_provider_extern_introspect!();
    fn is_reachable_non_generic_provider_extern (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { tcx . reachable_non_generics (def_id . krate) . contains_key (& def_id) }
}

macro_rules! exported_non_generic_symbols_provider_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exported_non_generic_symbols_provider_local in module {}", module_path!());
    };
}

mkfn!{
    exported_non_generic_symbols_provider_local_introspect!();
    fn exported_non_generic_symbols_provider_local < 'tcx > (tcx : TyCtxt < 'tcx > , _ : LocalCrate ,) -> & 'tcx [(ExportedSymbol < 'tcx > , SymbolExportInfo)] { if ! tcx . sess . opts . output_types . should_codegen () && ! tcx . is_sdylib_interface_build () { return & [] ; } let sorted = tcx . with_stable_hashing_context (| hcx | { tcx . reachable_non_generics (LOCAL_CRATE) . to_sorted (& hcx , true) }) ; let mut symbols : Vec < _ > = sorted . iter () . map (| & (& def_id , & info) | (ExportedSymbol :: NonGeneric (def_id) , info)) . collect () ; if ! tcx . sess . target . dll_tls_export { symbols . extend (sorted . iter () . filter_map (| & (& def_id , & info) | { tcx . needs_thread_local_shim (def_id) . then (| | { (ExportedSymbol :: ThreadLocalShim (def_id) , SymbolExportInfo { level : info . level , kind : SymbolExportKind :: Text , used : info . used , rustc_std_internal_symbol : info . rustc_std_internal_symbol , } ,) }) })) } if tcx . entry_fn (()) . is_some () { let exported_symbol = ExportedSymbol :: NoDefId (SymbolName :: new (tcx , tcx . sess . target . entry_name . as_ref ())) ; symbols . push ((exported_symbol , SymbolExportInfo { level : SymbolExportLevel :: C , kind : SymbolExportKind :: Text , used : false , rustc_std_internal_symbol : false , } ,)) ; } symbols . sort_by_cached_key (| s | s . 0 . symbol_name_for_local_instance (tcx)) ; tcx . arena . alloc_from_iter (symbols) }
}

macro_rules! exported_generic_symbols_provider_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exported_generic_symbols_provider_local in module {}", module_path!());
    };
}

mkfn!{
    exported_generic_symbols_provider_local_introspect!();
    fn exported_generic_symbols_provider_local < 'tcx > (tcx : TyCtxt < 'tcx > , _ : LocalCrate ,) -> & 'tcx [(ExportedSymbol < 'tcx > , SymbolExportInfo)] { if ! tcx . sess . opts . output_types . should_codegen () && ! tcx . is_sdylib_interface_build () { return & [] ; } let mut symbols : Vec < _ > = vec ! [] ; if tcx . local_crate_exports_generics () { use rustc_hir :: attrs :: Linkage ; use rustc_middle :: mir :: mono :: { MonoItem , Visibility } ; use rustc_middle :: ty :: InstanceKind ; let need_visibility = tcx . sess . target . dynamic_linking && ! tcx . sess . target . only_cdylib ; let cgus = tcx . collect_and_partition_mono_items (()) . codegen_units ; let reachable_set = tcx . reachable_set (()) ; let is_local_to_current_crate = | ty : Ty < '_ > | { let no_refs = ty . peel_refs () ; let root_def_id = match no_refs . kind () { ty :: Closure (closure , _) => * closure , ty :: FnDef (def_id , _) => * def_id , ty :: Coroutine (def_id , _) => * def_id , ty :: CoroutineClosure (def_id , _) => * def_id , ty :: CoroutineWitness (def_id , _) => * def_id , _ => return false , } ; let Some (root_def_id) = root_def_id . as_local () else { return false ; } ; let is_local = ! reachable_set . contains (& root_def_id) ; is_local } ; let is_instantiable_downstream = | did : Option < DefId > , generic_args : GenericArgsRef < 'tcx > | { generic_args . types () . chain (did . into_iter () . map (move | did | tcx . type_of (did) . skip_binder ())) . all (move | arg | { arg . walk () . all (| ty | { ty . as_type () . map_or (true , | ty | ! is_local_to_current_crate (ty)) }) }) } ; # [allow (rustc :: potential_query_instability)] for (mono_item , data) in cgus . iter () . flat_map (| cgu | cgu . items () . iter ()) { if data . linkage != Linkage :: External { continue ; } if need_visibility && data . visibility == Visibility :: Hidden { continue ; } if ! tcx . sess . opts . share_generics () { if tcx . codegen_fn_attrs (mono_item . def_id ()) . inline == rustc_hir :: attrs :: InlineAttr :: Never { } else { continue ; } } match * mono_item { MonoItem :: Fn (Instance { def : InstanceKind :: Item (def) , args }) => { let has_generics = args . non_erasable_generics () . next () . is_some () ; let should_export = has_generics && is_instantiable_downstream (Some (def) , & args) ; if should_export { let symbol = ExportedSymbol :: Generic (def , args) ; symbols . push ((symbol , SymbolExportInfo { level : SymbolExportLevel :: Rust , kind : SymbolExportKind :: Text , used : false , rustc_std_internal_symbol : false , } ,)) ; } } MonoItem :: Fn (Instance { def : InstanceKind :: DropGlue (_ , Some (ty)) , args }) => { assert_eq ! (args . non_erasable_generics () . next () , Some (GenericArgKind :: Type (ty))) ; let should_export = match ty . kind () { ty :: Adt (_ , args) => is_instantiable_downstream (None , args) , ty :: Closure (_ , args) => is_instantiable_downstream (None , args) , _ => true , } ; if should_export { symbols . push ((ExportedSymbol :: DropGlue (ty) , SymbolExportInfo { level : SymbolExportLevel :: Rust , kind : SymbolExportKind :: Text , used : false , rustc_std_internal_symbol : false , } ,)) ; } } MonoItem :: Fn (Instance { def : InstanceKind :: AsyncDropGlueCtorShim (_ , ty) , args , }) => { assert_eq ! (args . non_erasable_generics () . next () , Some (GenericArgKind :: Type (ty))) ; symbols . push ((ExportedSymbol :: AsyncDropGlueCtorShim (ty) , SymbolExportInfo { level : SymbolExportLevel :: Rust , kind : SymbolExportKind :: Text , used : false , rustc_std_internal_symbol : false , } ,)) ; } MonoItem :: Fn (Instance { def : InstanceKind :: AsyncDropGlue (def , ty) , args : _ }) => { symbols . push ((ExportedSymbol :: AsyncDropGlue (def , ty) , SymbolExportInfo { level : SymbolExportLevel :: Rust , kind : SymbolExportKind :: Text , used : false , rustc_std_internal_symbol : false , } ,)) ; } _ => { } } } } symbols . sort_by_cached_key (| s | s . 0 . symbol_name_for_local_instance (tcx)) ; tcx . arena . alloc_from_iter (symbols) }
}

macro_rules! upstream_monomorphizations_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function upstream_monomorphizations_provider in module {}", module_path!());
    };
}

mkfn!{
    upstream_monomorphizations_provider_introspect!();
    fn upstream_monomorphizations_provider (tcx : TyCtxt < '_ > , () : () ,) -> DefIdMap < UnordMap < GenericArgsRef < '_ > , CrateNum > > { let cnums = tcx . crates (()) ; let mut instances : DefIdMap < UnordMap < _ , _ > > = Default :: default () ; let drop_in_place_fn_def_id = tcx . lang_items () . drop_in_place_fn () ; let async_drop_in_place_fn_def_id = tcx . lang_items () . async_drop_in_place_fn () ; for & cnum in cnums . iter () { for (exported_symbol , _) in tcx . exported_generic_symbols (cnum) . iter () { let (def_id , args) = match * exported_symbol { ExportedSymbol :: Generic (def_id , args) => (def_id , args) , ExportedSymbol :: DropGlue (ty) => { if let Some (drop_in_place_fn_def_id) = drop_in_place_fn_def_id { (drop_in_place_fn_def_id , tcx . mk_args (& [ty . into ()])) } else { continue ; } } ExportedSymbol :: AsyncDropGlueCtorShim (ty) => { if let Some (async_drop_in_place_fn_def_id) = async_drop_in_place_fn_def_id { (async_drop_in_place_fn_def_id , tcx . mk_args (& [ty . into ()])) } else { continue ; } } ExportedSymbol :: AsyncDropGlue (def_id , ty) => (def_id , tcx . mk_args (& [ty . into ()])) , ExportedSymbol :: NonGeneric (..) | ExportedSymbol :: ThreadLocalShim (..) | ExportedSymbol :: NoDefId (..) => unreachable ! ("{exported_symbol:?}") , } ; let args_map = instances . entry (def_id) . or_default () ; match args_map . entry (args) { Occupied (mut e) => { let other_cnum = * e . get () ; if tcx . stable_crate_id (other_cnum) > tcx . stable_crate_id (cnum) { e . insert (cnum) ; } } Vacant (e) => { e . insert (cnum) ; } } } } instances }
}

macro_rules! upstream_monomorphizations_for_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function upstream_monomorphizations_for_provider in module {}", module_path!());
    };
}

mkfn!{
    upstream_monomorphizations_for_provider_introspect!();
    fn upstream_monomorphizations_for_provider (tcx : TyCtxt < '_ > , def_id : DefId ,) -> Option < & UnordMap < GenericArgsRef < '_ > , CrateNum > > { assert ! (! def_id . is_local ()) ; tcx . upstream_monomorphizations (()) . get (& def_id) }
}

macro_rules! upstream_drop_glue_for_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function upstream_drop_glue_for_provider in module {}", module_path!());
    };
}

mkfn!{
    upstream_drop_glue_for_provider_introspect!();
    fn upstream_drop_glue_for_provider < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> Option < CrateNum > { let def_id = tcx . lang_items () . drop_in_place_fn () ? ; tcx . upstream_monomorphizations_for (def_id) ? . get (& args) . cloned () }
}

macro_rules! upstream_async_drop_glue_for_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function upstream_async_drop_glue_for_provider in module {}", module_path!());
    };
}

mkfn!{
    upstream_async_drop_glue_for_provider_introspect!();
    fn upstream_async_drop_glue_for_provider < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> Option < CrateNum > { let def_id = tcx . lang_items () . async_drop_in_place_fn () ? ; tcx . upstream_monomorphizations_for (def_id) ? . get (& args) . cloned () }
}

macro_rules! is_unreachable_local_definition_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_unreachable_local_definition_provider in module {}", module_path!());
    };
}

mkfn!{
    is_unreachable_local_definition_provider_introspect!();
    fn is_unreachable_local_definition_provider (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { ! tcx . reachable_set (()) . contains (& def_id) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { providers . reachable_non_generics = reachable_non_generics_provider ; providers . is_reachable_non_generic = is_reachable_non_generic_provider_local ; providers . exported_non_generic_symbols = exported_non_generic_symbols_provider_local ; providers . exported_generic_symbols = exported_generic_symbols_provider_local ; providers . upstream_monomorphizations = upstream_monomorphizations_provider ; providers . is_unreachable_local_definition = is_unreachable_local_definition_provider ; providers . upstream_drop_glue_for = upstream_drop_glue_for_provider ; providers . upstream_async_drop_glue_for = upstream_async_drop_glue_for_provider ; providers . wasm_import_module_map = wasm_import_module_map ; providers . extern_queries . is_reachable_non_generic = is_reachable_non_generic_provider_extern ; providers . extern_queries . upstream_monomorphizations_for = upstream_monomorphizations_for_provider ; }
}

macro_rules! allocator_shim_symbols_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function allocator_shim_symbols in module {}", module_path!());
    };
}

mkfn!{
    allocator_shim_symbols_introspect!();
    pub (crate) fn allocator_shim_symbols (tcx : TyCtxt < '_ > ,) -> impl Iterator < Item = (String , SymbolExportKind) > { ALLOCATOR_METHODS . iter () . map (move | method | mangle_internal_symbol (tcx , global_fn_name (method . name) . as_str ())) . chain ([mangle_internal_symbol (tcx , "__rust_alloc_error_handler") , mangle_internal_symbol (tcx , OomStrategy :: SYMBOL) , mangle_internal_symbol (tcx , NO_ALLOC_SHIM_IS_UNSTABLE) ,]) . map (move | symbol_name | { let exported_symbol = ExportedSymbol :: NoDefId (SymbolName :: new (tcx , & symbol_name)) ; (symbol_export :: exporting_symbol_name_for_instance_in_crate (tcx , exported_symbol , LOCAL_CRATE ,) , SymbolExportKind :: Text ,) }) }
}

macro_rules! symbol_export_level_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symbol_export_level in module {}", module_path!());
    };
}

mkfn!{
    symbol_export_level_introspect!();
    fn symbol_export_level (tcx : TyCtxt < '_ > , sym_def_id : DefId) -> SymbolExportLevel { let codegen_fn_attrs = tcx . codegen_fn_attrs (sym_def_id) ; let is_extern = codegen_fn_attrs . contains_extern_indicator () ; let std_internal = codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL) ; if is_extern && ! std_internal { let target = & tcx . sess . target . llvm_target ; if target . contains ("emscripten") { if let DefKind :: Static { .. } = tcx . def_kind (sym_def_id) { return SymbolExportLevel :: Rust ; } } SymbolExportLevel :: C } else { SymbolExportLevel :: Rust } }
}

macro_rules! symbol_name_for_instance_in_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symbol_name_for_instance_in_crate in module {}", module_path!());
    };
}

mkfn!{
    symbol_name_for_instance_in_crate_introspect!();
    # [doc = " This is the symbol name of the given instance instantiated in a specific crate."] pub (crate) fn symbol_name_for_instance_in_crate < 'tcx > (tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > , instantiating_crate : CrateNum ,) -> String { if instantiating_crate == LOCAL_CRATE { return symbol . symbol_name_for_local_instance (tcx) . to_string () ; } match symbol { ExportedSymbol :: NonGeneric (def_id) => { rustc_symbol_mangling :: symbol_name_for_instance_in_crate (tcx , Instance :: mono (tcx , def_id) , instantiating_crate ,) } ExportedSymbol :: Generic (def_id , args) => { rustc_symbol_mangling :: symbol_name_for_instance_in_crate (tcx , Instance :: new_raw (def_id , args) , instantiating_crate ,) } ExportedSymbol :: ThreadLocalShim (def_id) => { rustc_symbol_mangling :: symbol_name_for_instance_in_crate (tcx , ty :: Instance { def : ty :: InstanceKind :: ThreadLocalShim (def_id) , args : ty :: GenericArgs :: empty () , } , instantiating_crate ,) } ExportedSymbol :: DropGlue (ty) => rustc_symbol_mangling :: symbol_name_for_instance_in_crate (tcx , Instance :: resolve_drop_in_place (tcx , ty) , instantiating_crate ,) , ExportedSymbol :: AsyncDropGlueCtorShim (ty) => { rustc_symbol_mangling :: symbol_name_for_instance_in_crate (tcx , Instance :: resolve_async_drop_in_place (tcx , ty) , instantiating_crate ,) } ExportedSymbol :: AsyncDropGlue (def_id , ty) => { rustc_symbol_mangling :: symbol_name_for_instance_in_crate (tcx , Instance :: resolve_async_drop_in_place_poll (tcx , def_id , ty) , instantiating_crate ,) } ExportedSymbol :: NoDefId (symbol_name) => symbol_name . to_string () , } }
}

macro_rules! calling_convention_for_symbol_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function calling_convention_for_symbol in module {}", module_path!());
    };
}

mkfn!{
    calling_convention_for_symbol_introspect!();
    fn calling_convention_for_symbol < 'tcx > (tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > ,) -> (CanonAbi , & 'tcx [rustc_target :: callconv :: ArgAbi < 'tcx , Ty < 'tcx > >]) { let instance = match symbol { ExportedSymbol :: NonGeneric (def_id) | ExportedSymbol :: Generic (def_id , _) if tcx . is_static (def_id) => { None } ExportedSymbol :: NonGeneric (def_id) => Some (Instance :: mono (tcx , def_id)) , ExportedSymbol :: Generic (def_id , args) => Some (Instance :: new_raw (def_id , args)) , ExportedSymbol :: DropGlue (..) => None , ExportedSymbol :: AsyncDropGlueCtorShim (..) => None , ExportedSymbol :: AsyncDropGlue (..) => None , ExportedSymbol :: NoDefId (..) => None , ExportedSymbol :: ThreadLocalShim (..) => None , } ; instance . map (| i | { tcx . fn_abi_of_instance (ty :: TypingEnv :: fully_monomorphized () . as_query_input ((i , ty :: List :: empty ())) ,) . unwrap_or_else (| _ | bug ! ("fn_abi_of_instance({i:?}) failed")) }) . map (| fnabi | (fnabi . conv , & fnabi . args [..])) . unwrap_or ((CanonAbi :: Rust , & [])) }
}

macro_rules! linking_symbol_name_for_instance_in_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function linking_symbol_name_for_instance_in_crate in module {}", module_path!());
    };
}

mkfn!{
    linking_symbol_name_for_instance_in_crate_introspect!();
    # [doc = " This is the symbol name of the given instance as seen by the linker."] # [doc = ""] # [doc = " On 32-bit Windows symbols are decorated according to their calling conventions."] pub (crate) fn linking_symbol_name_for_instance_in_crate < 'tcx > (tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > , export_kind : SymbolExportKind , instantiating_crate : CrateNum ,) -> String { let mut undecorated = symbol_name_for_instance_in_crate (tcx , symbol , instantiating_crate) ; if let Some (name) = maybe_emutls_symbol_name (tcx , symbol , & undecorated) { return name ; } let target = & tcx . sess . target ; if ! target . is_like_windows { return undecorated ; } let prefix = match & target . arch [..] { "x86" => Some ('_') , "x86_64" => None , "arm64ec" if export_kind == SymbolExportKind :: Text => Some ('#') , _ => return undecorated , } ; let (callconv , args) = calling_convention_for_symbol (tcx , symbol) ; let (prefix , suffix) = match callconv { CanonAbi :: X86 (X86Call :: Fastcall) => ("@" , "@") , CanonAbi :: X86 (X86Call :: Stdcall) => ("_" , "@") , CanonAbi :: X86 (X86Call :: Vectorcall) => ("" , "@@") , _ => { if let Some (prefix) = prefix { undecorated . insert (0 , prefix) ; } return undecorated ; } } ; let args_in_bytes : u64 = args . iter () . map (| abi | abi . layout . size . bytes () . next_multiple_of (target . pointer_width as u64 / 8)) . sum () ; format ! ("{prefix}{undecorated}{suffix}{args_in_bytes}") }
}

macro_rules! exporting_symbol_name_for_instance_in_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exporting_symbol_name_for_instance_in_crate in module {}", module_path!());
    };
}

mkfn!{
    exporting_symbol_name_for_instance_in_crate_introspect!();
    pub (crate) fn exporting_symbol_name_for_instance_in_crate < 'tcx > (tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > , cnum : CrateNum ,) -> String { let undecorated = symbol_name_for_instance_in_crate (tcx , symbol , cnum) ; maybe_emutls_symbol_name (tcx , symbol , & undecorated) . unwrap_or (undecorated) }
}

macro_rules! extend_exported_symbols_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extend_exported_symbols in module {}", module_path!());
    };
}

mkfn!{
    extend_exported_symbols_introspect!();
    # [doc = " On amdhsa, `gpu-kernel` functions have an associated metadata object with a `.kd` suffix."] # [doc = " Add it to the symbols list for all kernel functions, so that it is exported in the linked"] # [doc = " object."] pub (crate) fn extend_exported_symbols < 'tcx > (symbols : & mut Vec < (String , SymbolExportKind) > , tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > , instantiating_crate : CrateNum ,) { let (callconv , _) = calling_convention_for_symbol (tcx , symbol) ; if callconv != CanonAbi :: GpuKernel || tcx . sess . target . os != "amdhsa" { return ; } let undecorated = symbol_name_for_instance_in_crate (tcx , symbol , instantiating_crate) ; symbols . push ((format ! ("{undecorated}.kd") , SymbolExportKind :: Data)) ; }
}

macro_rules! maybe_emutls_symbol_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_emutls_symbol_name in module {}", module_path!());
    };
}

mkfn!{
    maybe_emutls_symbol_name_introspect!();
    fn maybe_emutls_symbol_name < 'tcx > (tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > , undecorated : & str ,) -> Option < String > { if matches ! (tcx . sess . tls_model () , TlsModel :: Emulated) && let ExportedSymbol :: NonGeneric (def_id) = symbol && tcx . is_thread_local_static (def_id) { Some (format ! ("__emutls_v.{undecorated}")) } else { None } }
}

macro_rules! wasm_import_module_map_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wasm_import_module_map in module {}", module_path!());
    };
}

mkfn!{
    wasm_import_module_map_introspect!();
    fn wasm_import_module_map (tcx : TyCtxt < '_ > , cnum : CrateNum) -> DefIdMap < String > { let native_libs = tcx . native_libraries (cnum) ; let def_id_to_native_lib = native_libs . iter () . filter_map (| lib | lib . foreign_module . map (| id | (id , lib))) . collect :: < DefIdMap < _ > > () ; let mut ret = DefIdMap :: default () ; for (def_id , lib) in tcx . foreign_modules (cnum) . iter () { let module = def_id_to_native_lib . get (def_id) . and_then (| s | s . wasm_import_module ()) ; let Some (module) = module else { continue } ; ret . extend (lib . foreign_items . iter () . map (| id | { assert_eq ! (id . krate , cnum) ; (* id , module . to_string ()) })) ; } ret }
}