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
mkuse!{use std :: env :: consts :: { DLL_PREFIX , DLL_SUFFIX } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: sync :: atomic :: { AtomicBool , Ordering } ;}
mkuse!{use std :: sync :: { Arc , OnceLock } ;}
mkuse!{use std :: { env , thread } ;}
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_attr_parsing :: { ShouldEmit , validate_attr } ;}
mkuse!{use rustc_codegen_ssa :: traits :: CodegenBackend ;}
mkuse!{use rustc_data_structures :: jobserver :: Proxy ;}
mkuse!{use rustc_data_structures :: sync ;}
mkuse!{use rustc_errors :: LintBuffer ;}
mkuse!{use rustc_metadata :: { DylibError , load_symbol_from_dylib } ;}
mkuse!{use rustc_middle :: ty :: CurrentGcx ;}
mkuse!{use rustc_session :: config :: { Cfg , OutFileName , OutputFilenames , OutputTypes , Sysroot , host_tuple } ;}
mkuse!{use rustc_session :: lint :: { self , BuiltinLintDiag } ;}
mkuse!{use rustc_session :: output :: { CRATE_TYPES , categorize_crate_type } ;}
mkuse!{use rustc_session :: { EarlyDiagCtxt , Session , filesearch } ;}
mkuse!{use rustc_span :: edit_distance :: find_best_match_for_name ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: source_map :: SourceMapInputs ;}
mkuse!{use rustc_span :: { SessionGlobals , Symbol , sym } ;}
mkuse!{use rustc_target :: spec :: Target ;}
mkuse!{use tracing :: info ;}
mkuse!{use crate :: errors ;}
mkuse!{use crate :: passes :: parse_crate_name ;}
mkitem!{#[doc = " Function pointer type that constructs a new CodegenBackend."] type MakeBackendFn = fn () -> Box < dyn CodegenBackend > ;}

macro_rules! add_configuration_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_configuration in module {}", module_path!());
    };
}

mkfn!{
    add_configuration_introspect!();
    #[doc = " Adds `target_feature = \"...\"` cfgs for a variety of platform"] #[doc = " specific features (SSE, NEON etc.)."] #[doc = ""] #[doc = " This is performed by checking whether a set of permitted features"] #[doc = " is available on the target machine, by querying the codegen backend."] pub (crate) fn add_configuration (cfg : & mut Cfg , sess : & mut Session , codegen_backend : & dyn CodegenBackend ,) { let tf = sym :: target_feature ; let tf_cfg = codegen_backend . target_config (sess) ; sess . unstable_target_features . extend (tf_cfg . unstable_target_features . iter () . copied ()) ; sess . target_features . extend (tf_cfg . target_features . iter () . copied ()) ; cfg . extend (tf_cfg . target_features . into_iter () . map (| feat | (tf , Some (feat)))) ; if tf_cfg . has_reliable_f16 { cfg . insert ((sym :: target_has_reliable_f16 , None)) ; } if tf_cfg . has_reliable_f16_math { cfg . insert ((sym :: target_has_reliable_f16_math , None)) ; } if tf_cfg . has_reliable_f128 { cfg . insert ((sym :: target_has_reliable_f128 , None)) ; } if tf_cfg . has_reliable_f128_math { cfg . insert ((sym :: target_has_reliable_f128_math , None)) ; } if sess . crt_static (None) { cfg . insert ((tf , Some (sym :: crt_dash_static))) ; } }
}

macro_rules! check_abi_required_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_abi_required_features in module {}", module_path!());
    };
}

mkfn!{
    check_abi_required_features_introspect!();
    #[doc = " Ensures that all target features required by the ABI are present."] #[doc = " Must be called after `unstable_target_features` has been populated!"] pub (crate) fn check_abi_required_features (sess : & Session) { let abi_feature_constraints = sess . target . abi_required_features () ; for feature in abi_feature_constraints . required . iter () . chain (abi_feature_constraints . incompatible . iter ()) { assert ! (sess . target . rust_target_features () . iter () . any (| (name , ..) | feature == name) , "target feature {feature} is required/incompatible for the current ABI but not a recognized feature for this target") ; } for feature in abi_feature_constraints . required { if ! sess . unstable_target_features . contains (& Symbol :: intern (feature)) { sess . dcx () . emit_warn (errors :: AbiRequiredTargetFeature { feature , enabled : "enabled" }) ; } } for feature in abi_feature_constraints . incompatible { if sess . unstable_target_features . contains (& Symbol :: intern (feature)) { sess . dcx () . emit_warn (errors :: AbiRequiredTargetFeature { feature , enabled : "disabled" }) ; } } }
}
mkitem!{pub static STACK_SIZE : OnceLock < usize > = OnceLock :: new () ;}
mkitem!{pub const DEFAULT_STACK_SIZE : usize = 8 * 1024 * 1024 ;}

macro_rules! init_stack_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init_stack_size in module {}", module_path!());
    };
}

mkfn!{
    init_stack_size_introspect!();
    fn init_stack_size (early_dcx : & EarlyDiagCtxt) -> usize { * STACK_SIZE . get_or_init (| | { env :: var_os ("RUST_MIN_STACK") . as_ref () . map (| os_str | os_str . to_string_lossy ()) . filter (| s | ! s . trim () . is_empty ()) . map (| s | { let s = s . trim () ; #[allow (rustc :: untranslatable_diagnostic , rustc :: diagnostic_outside_of_impl)] s . parse :: < usize > () . unwrap_or_else (| _ | { let mut err = early_dcx . early_struct_fatal (format ! (r#"`RUST_MIN_STACK` should be a number of bytes, but was "{s}""# ,)) ; err . note ("you can also unset `RUST_MIN_STACK` to use the default stack size") ; err . emit () }) }) . unwrap_or (DEFAULT_STACK_SIZE) }) }
}

macro_rules! run_in_thread_with_globals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_in_thread_with_globals in module {}", module_path!());
    };
}

mkfn!{
    run_in_thread_with_globals_introspect!();
    fn run_in_thread_with_globals < F : FnOnce (CurrentGcx , Arc < Proxy >) -> R + Send , R : Send > (thread_stack_size : usize , edition : Edition , sm_inputs : SourceMapInputs , extra_symbols : & [& 'static str] , f : F ,) -> R { let builder = thread :: Builder :: new () . name ("rustc" . to_string ()) . stack_size (thread_stack_size) ; thread :: scope (| s | { let r = builder . spawn_scoped (s , move | | { rustc_span :: create_session_globals_then (edition , extra_symbols , Some (sm_inputs) , | | f (CurrentGcx :: new () , Proxy :: new ()) ,) }) . unwrap () . join () ; match r { Ok (v) => v , Err (e) => std :: panic :: resume_unwind (e) , } }) }
}

macro_rules! run_in_thread_pool_with_globals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_in_thread_pool_with_globals in module {}", module_path!());
    };
}

mkfn!{
    run_in_thread_pool_with_globals_introspect!();
    pub (crate) fn run_in_thread_pool_with_globals < F : FnOnce (CurrentGcx , Arc < Proxy >) -> R + Send , R : Send , > (thread_builder_diag : & EarlyDiagCtxt , edition : Edition , threads : usize , extra_symbols : & [& 'static str] , sm_inputs : SourceMapInputs , f : F ,) -> R { use std :: process ; use rustc_data_structures :: defer ; use rustc_data_structures :: sync :: FromDyn ; use rustc_middle :: ty :: tls ; use rustc_query_impl :: QueryCtxt ; use rustc_query_system :: query :: { QueryContext , break_query_cycles } ; let thread_stack_size = init_stack_size (thread_builder_diag) ; let registry = sync :: Registry :: new (std :: num :: NonZero :: new (threads) . unwrap ()) ; if ! sync :: is_dyn_thread_safe () { return run_in_thread_with_globals (thread_stack_size , edition , sm_inputs , extra_symbols , | current_gcx , jobserver_proxy | { registry . register () ; f (current_gcx , jobserver_proxy) } ,) ; } let current_gcx = FromDyn :: from (CurrentGcx :: new ()) ; let current_gcx2 = current_gcx . clone () ; let proxy = Proxy :: new () ; let proxy_ = Arc :: clone (& proxy) ; let proxy__ = Arc :: clone (& proxy) ; let builder = rustc_thread_pool :: ThreadPoolBuilder :: new () . thread_name (| _ | "rustc" . to_string ()) . acquire_thread_handler (move | | proxy_ . acquire_thread ()) . release_thread_handler (move | | proxy__ . release_thread ()) . num_threads (threads) . deadlock_handler (move | | { let current_gcx2 = current_gcx2 . clone () ; let registry = rustc_thread_pool :: Registry :: current () ; let session_globals = rustc_span :: with_session_globals (| session_globals | { session_globals as * const SessionGlobals as usize }) ; thread :: Builder :: new () . name ("rustc query cycle handler" . to_string ()) . spawn (move | | { let on_panic = defer (| | { eprintln ! ("internal compiler error: query cycle handler thread panicked, aborting process") ; process :: abort () ; }) ; current_gcx2 . access (| gcx | { tls :: enter_context (& tls :: ImplicitCtxt :: new (gcx) , | | { tls :: with (| tcx | { let query_map = rustc_span :: set_session_globals_then (unsafe { & * (session_globals as * const SessionGlobals) } , | | { QueryCtxt :: new (tcx) . collect_active_jobs () . ok () . expect ("failed to collect active queries in deadlock handler") }) ; break_query_cycles (query_map , & registry) ; }) }) }) ; on_panic . disable () ; }) . unwrap () ; }) . stack_size (thread_stack_size) ; rustc_span :: create_session_globals_then (edition , extra_symbols , Some (sm_inputs) , | | { rustc_span :: with_session_globals (| session_globals | { let session_globals = FromDyn :: from (session_globals) ; builder . build_scoped (move | thread : rustc_thread_pool :: ThreadBuilder | { registry . register () ; rustc_span :: set_session_globals_then (session_globals . into_inner () , | | { thread . run () }) } , move | pool : & rustc_thread_pool :: ThreadPool | { pool . install (| | f (current_gcx . into_inner () , proxy)) } ,) . unwrap () }) }) }
}

macro_rules! load_backend_from_dylib_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function load_backend_from_dylib in module {}", module_path!());
    };
}

mkfn!{
    load_backend_from_dylib_introspect!();
    #[allow (rustc :: untranslatable_diagnostic)] fn load_backend_from_dylib (early_dcx : & EarlyDiagCtxt , path : & Path) -> MakeBackendFn { match unsafe { load_symbol_from_dylib :: < MakeBackendFn > (path , "__rustc_codegen_backend") } { Ok (backend_sym) => backend_sym , Err (DylibError :: DlOpen (path , err)) => { let err = format ! ("couldn't load codegen backend {path}{err}") ; early_dcx . early_fatal (err) ; } Err (DylibError :: DlSym (_path , err)) => { let e = format ! ("`__rustc_codegen_backend` symbol lookup in the codegen backend failed{err}" ,) ; early_dcx . early_fatal (e) ; } } }
}

macro_rules! get_codegen_backend_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_codegen_backend in module {}", module_path!());
    };
}

mkfn!{
    get_codegen_backend_introspect!();
    #[doc = " Get the codegen backend based on the name and specified sysroot."] #[doc = ""] #[doc = " A name of `None` indicates that the default backend should be used."] pub fn get_codegen_backend (early_dcx : & EarlyDiagCtxt , sysroot : & Sysroot , backend_name : Option < & str > , target : & Target ,) -> Box < dyn CodegenBackend > { static LOAD : OnceLock < unsafe fn () -> Box < dyn CodegenBackend > > = OnceLock :: new () ; let load = LOAD . get_or_init (| | { let backend = backend_name . or (target . default_codegen_backend . as_deref ()) . or (option_env ! ("CFG_DEFAULT_CODEGEN_BACKEND")) . unwrap_or ("llvm") ; match backend { filename if filename . contains ('.') => { load_backend_from_dylib (early_dcx , filename . as_ref ()) } #[cfg (feature = "llvm")] "llvm" => rustc_codegen_llvm :: LlvmCodegenBackend :: new , backend_name => get_codegen_sysroot (early_dcx , sysroot , backend_name) , } }) ; unsafe { load () } }
}

macro_rules! rustc_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rustc_path in module {}", module_path!());
    };
}

mkfn!{
    rustc_path_introspect!();
    pub fn rustc_path < 'a > (sysroot : & Sysroot) -> Option < & 'a Path > { static RUSTC_PATH : OnceLock < Option < PathBuf > > = OnceLock :: new () ; RUSTC_PATH . get_or_init (| | { let candidate = sysroot . default . join (env ! ("RUSTC_INSTALL_BINDIR")) . join (if cfg ! (target_os = "windows") { "rustc.exe" } else { "rustc" }) ; candidate . exists () . then_some (candidate) }) . as_deref () }
}

macro_rules! get_codegen_sysroot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_codegen_sysroot in module {}", module_path!());
    };
}

mkfn!{
    get_codegen_sysroot_introspect!();
    #[allow (rustc :: untranslatable_diagnostic)] fn get_codegen_sysroot (early_dcx : & EarlyDiagCtxt , sysroot : & Sysroot , backend_name : & str ,) -> MakeBackendFn { static LOADED : AtomicBool = AtomicBool :: new (false) ; assert ! (! LOADED . fetch_or (true , Ordering :: SeqCst) , "cannot load the default codegen backend twice") ; let target = host_tuple () ; let sysroot = sysroot . all_paths () . map (| sysroot | { filesearch :: make_target_lib_path (sysroot , target) . with_file_name ("codegen-backends") }) . find (| f | { info ! ("codegen backend candidate: {}" , f . display ()) ; f . exists () }) . unwrap_or_else (| | { let candidates = sysroot . all_paths () . map (| p | p . display () . to_string ()) . collect :: < Vec < _ > > () . join ("\n* ") ; let err = format ! ("failed to find a `codegen-backends` folder \
                           in the sysroot candidates:\n* {candidates}") ; early_dcx . early_fatal (err) ; }) ; info ! ("probing {} for a codegen backend" , sysroot . display ()) ; let d = sysroot . read_dir () . unwrap_or_else (| e | { let err = format ! ("failed to load default codegen backend, couldn't \
                           read `{}`: {}" , sysroot . display () , e) ; early_dcx . early_fatal (err) ; }) ; let mut file : Option < PathBuf > = None ; let expected_names = & [format ! ("rustc_codegen_{}-{}" , backend_name , env ! ("CFG_RELEASE")) , format ! ("rustc_codegen_{backend_name}") ,] ; for entry in d . filter_map (| e | e . ok ()) { let path = entry . path () ; let Some (filename) = path . file_name () . and_then (| s | s . to_str ()) else { continue } ; if ! (filename . starts_with (DLL_PREFIX) && filename . ends_with (DLL_SUFFIX)) { continue ; } let name = & filename [DLL_PREFIX . len () .. filename . len () - DLL_SUFFIX . len ()] ; if ! expected_names . iter () . any (| expected | expected == name) { continue ; } if let Some (ref prev) = file { let err = format ! ("duplicate codegen backends found\n\
                               first:  {}\n\
                               second: {}\n\
            " , prev . display () , path . display ()) ; early_dcx . early_fatal (err) ; } file = Some (path . clone ()) ; } match file { Some (ref s) => load_backend_from_dylib (early_dcx , s) , None => { let err = format ! ("unsupported builtin codegen backend `{backend_name}`") ; early_dcx . early_fatal (err) ; } } }
}

macro_rules! check_attr_crate_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_attr_crate_type in module {}", module_path!());
    };
}

mkfn!{
    check_attr_crate_type_introspect!();
    pub (crate) fn check_attr_crate_type (sess : & Session , attrs : & [ast :: Attribute] , lint_buffer : & mut LintBuffer ,) { for a in attrs . iter () { if a . has_name (sym :: crate_type) { if let Some (n) = a . value_str () { if categorize_crate_type (n) . is_some () { return ; } if let ast :: MetaItemKind :: NameValue (spanned) = a . meta_kind () . unwrap () { let span = spanned . span ; let candidate = find_best_match_for_name (& CRATE_TYPES . iter () . map (| (k , _) | * k) . collect :: < Vec < _ > > () , n , None ,) ; lint_buffer . buffer_lint (lint :: builtin :: UNKNOWN_CRATE_TYPES , ast :: CRATE_NODE_ID , span , BuiltinLintDiag :: UnknownCrateTypes { span , candidate } ,) ; } } else { validate_attr :: emit_fatal_malformed_builtin_attribute (& sess . psess , a , sym :: crate_type ,) ; } } } }
}

macro_rules! multiple_output_types_to_stdout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function multiple_output_types_to_stdout in module {}", module_path!());
    };
}

mkfn!{
    multiple_output_types_to_stdout_introspect!();
    fn multiple_output_types_to_stdout (output_types : & OutputTypes , single_output_file_is_stdout : bool ,) -> bool { use std :: io :: IsTerminal ; if std :: io :: stdout () . is_terminal () { let named_text_types = output_types . iter () . filter (| (f , o) | f . is_text_output () && * o == & Some (OutFileName :: Stdout)) . count () ; let unnamed_text_types = output_types . iter () . filter (| (f , o) | f . is_text_output () && o . is_none ()) . count () ; named_text_types > 1 || unnamed_text_types > 1 && single_output_file_is_stdout } else { let named_types = output_types . values () . filter (| o | * o == & Some (OutFileName :: Stdout)) . count () ; let unnamed_types = output_types . values () . filter (| o | o . is_none ()) . count () ; named_types > 1 || unnamed_types > 1 && single_output_file_is_stdout } }
}

macro_rules! build_output_filenames_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_output_filenames in module {}", module_path!());
    };
}

mkfn!{
    build_output_filenames_introspect!();
    pub fn build_output_filenames (attrs : & [ast :: Attribute] , sess : & Session) -> OutputFilenames { if multiple_output_types_to_stdout (& sess . opts . output_types , sess . io . output_file == Some (OutFileName :: Stdout) ,) { sess . dcx () . emit_fatal (errors :: MultipleOutputTypesToStdout) ; } let crate_name = sess . opts . crate_name . clone () . or_else (| | { parse_crate_name (sess , attrs , ShouldEmit :: Nothing) . map (| i | i . 0 . to_string ()) }) ; match sess . io . output_file { None => { let dirpath = sess . io . output_dir . clone () . unwrap_or_default () ; let stem = crate_name . clone () . unwrap_or_else (| | sess . io . input . filestem () . to_owned ()) ; OutputFilenames :: new (dirpath , crate_name . unwrap_or_else (| | stem . replace ('-' , "_")) , stem , None , sess . io . temps_dir . clone () , sess . opts . cg . extra_filename . clone () , sess . opts . output_types . clone () ,) } Some (ref out_file) => { let unnamed_output_types = sess . opts . output_types . values () . filter (| a | a . is_none ()) . count () ; let ofile = if unnamed_output_types > 1 { sess . dcx () . emit_warn (errors :: MultipleOutputTypesAdaption) ; None } else { if ! sess . opts . cg . extra_filename . is_empty () { sess . dcx () . emit_warn (errors :: IgnoringExtraFilename) ; } Some (out_file . clone ()) } ; if sess . io . output_dir != None { sess . dcx () . emit_warn (errors :: IgnoringOutDir) ; } let out_filestem = out_file . filestem () . unwrap_or_default () . to_str () . unwrap () . to_string () ; OutputFilenames :: new (out_file . parent () . unwrap_or_else (| | Path :: new ("")) . to_path_buf () , crate_name . unwrap_or_else (| | out_filestem . replace ('-' , "_")) , out_filestem , ofile , sess . io . temps_dir . clone () , sess . opts . cg . extra_filename . clone () , sess . opts . output_types . clone () ,) } } }
}
mkitem!{#[doc = " Returns a version string such as \"1.46.0 (04488afe3 2020-08-24)\" when invoked by an in-tree tool."] pub macro version_str () { option_env ! ("CFG_VERSION") }}

macro_rules! rustc_version_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rustc_version_str in module {}", module_path!());
    };
}

mkfn!{
    rustc_version_str_introspect!();
    #[doc = " Returns the version string for `rustc` itself (which may be different from a tool version)."] pub fn rustc_version_str () -> Option < & 'static str > { version_str ! () }
}