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
mkuse!{use std :: collections :: BTreeMap ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: process :: { Command , Stdio } ;}
mkuse!{use std :: { env , fs , str } ;}
mkitem!{#[doc = " Static library that will be built"] const LIB_NAME : & str = "musl_math_prefixed" ;}
mkitem!{#[doc = " Files that have more than one symbol. Map of file names to the symbols defined in that file."] const MULTIPLE_SYMBOLS : & [(& str , & [& str])] = & [("__invtrigl" , & ["__invtrigl" , "__invtrigl_R" , "__pio2_hi" , "__pio2_lo"] ,) , ("__polevll" , & ["__polevll" , "__p1evll"]) , ("erf" , & ["erf" , "erfc"]) , ("erff" , & ["erff" , "erfcf"]) , ("erfl" , & ["erfl" , "erfcl"]) , ("exp10" , & ["exp10" , "pow10"]) , ("exp10f" , & ["exp10f" , "pow10f"]) , ("exp10l" , & ["exp10l" , "pow10l"]) , ("exp2f_data" , & ["exp2f_data" , "__exp2f_data"]) , ("exp_data" , & ["exp_data" , "__exp_data"]) , ("j0" , & ["j0" , "y0"]) , ("j0f" , & ["j0f" , "y0f"]) , ("j1" , & ["j1" , "y1"]) , ("j1f" , & ["j1f" , "y1f"]) , ("jn" , & ["jn" , "yn"]) , ("jnf" , & ["jnf" , "ynf"]) , ("lgamma" , & ["lgamma" , "__lgamma_r"]) , ("remainder" , & ["remainder" , "drem"]) , ("remainderf" , & ["remainderf" , "dremf"]) , ("lgammaf" , & ["lgammaf" , "lgammaf_r" , "__lgammaf_r"]) , ("lgammal" , & ["lgammal" , "lgammal_r" , "__lgammal_r"]) , ("log2_data" , & ["log2_data" , "__log2_data"]) , ("log2f_data" , & ["log2f_data" , "__log2f_data"]) , ("log_data" , & ["log_data" , "__log_data"]) , ("logf_data" , & ["logf_data" , "__logf_data"]) , ("pow_data" , & ["pow_data" , "__pow_log_data"]) , ("powf_data" , & ["powf_data" , "__powf_log2_data"]) , ("signgam" , & ["signgam" , "__signgam"]) , ("sqrt_data" , & ["sqrt_data" , "__rsqrt_tab"]) ,] ;}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { let cfg = Config :: from_env () ; if cfg . target_env == "msvc" || cfg . target_family == "wasm" || cfg . target_features . iter () . any (| f | f == "thumb-mode") { println ! ("cargo::warning=Musl doesn't compile with the current \
            target {}; skipping build" , & cfg . target_string) ; return ; } build_musl_math (& cfg) ; }
}
mkitem!{mkstruct!{#[allow (dead_code)] #[derive (Debug)] struct Config { manifest_dir : PathBuf , out_dir : PathBuf , musl_dir : PathBuf , musl_arch : String , target_arch : String , target_env : String , target_family : String , target_os : String , target_string : String , target_vendor : String , target_features : Vec < String > , }}}
mkitem!{mkimpl!{impl Config { fn from_env () -> Self { let manifest_dir = PathBuf :: from (env :: var ("CARGO_MANIFEST_DIR") . unwrap ()) ; let target_features = env :: var ("CARGO_CFG_TARGET_FEATURE") . map (| feats | feats . split (',') . map (ToOwned :: to_owned) . collect ()) . unwrap_or_default () ; let musl_dir = manifest_dir . join ("musl") ; let target_arch = env :: var ("CARGO_CFG_TARGET_ARCH") . unwrap () ; let musl_arch = if target_arch == "x86" { "i386" . to_owned () } else { target_arch . clone () } ; println ! ("cargo::rerun-if-changed={}/c_patches" , manifest_dir . display ()) ; println ! ("cargo::rerun-if-changed={}" , musl_dir . display ()) ; Self { manifest_dir , out_dir : PathBuf :: from (env :: var ("OUT_DIR") . unwrap ()) , musl_dir , musl_arch , target_arch , target_env : env :: var ("CARGO_CFG_TARGET_ENV") . unwrap () , target_family : env :: var ("CARGO_CFG_TARGET_FAMILY") . unwrap () , target_os : env :: var ("CARGO_CFG_TARGET_OS") . unwrap () , target_string : env :: var ("TARGET") . unwrap () , target_vendor : env :: var ("CARGO_CFG_TARGET_VENDOR") . unwrap () , target_features , } } }}}

macro_rules! build_musl_math_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_musl_math in module {}", module_path!());
    };
}

mkfn!{
    build_musl_math_introspect!();
    #[doc = " Build musl math symbols to a static library"] fn build_musl_math (cfg : & Config) { let musl_dir = & cfg . musl_dir ; let math = musl_dir . join ("src/math") ; let arch_dir = musl_dir . join ("arch") . join (& cfg . musl_arch) ; assert ! (math . exists () , "musl source not found. You may need to run `./ci/update-musl.sh`.") ; let source_map = find_math_source (& math , cfg) ; let out_path = cfg . out_dir . join (format ! ("lib{LIB_NAME}.a")) ; let obj_include = cfg . out_dir . join ("musl_obj/include") ; fs :: create_dir_all (& obj_include) . unwrap () ; fs :: create_dir_all (obj_include . join ("bits")) . unwrap () ; let sed_stat = Command :: new ("sed") . arg ("-f") . arg (musl_dir . join ("tools/mkalltypes.sed")) . arg (arch_dir . join ("bits/alltypes.h.in")) . arg (musl_dir . join ("include/alltypes.h.in")) . stderr (Stdio :: inherit ()) . output () . unwrap () ; assert ! (sed_stat . status . success () , "sed command failed: {:?}" , sed_stat . status) ; fs :: write (obj_include . join ("bits/alltypes.h") , sed_stat . stdout) . unwrap () ; let mut cbuild = cc :: Build :: new () ; cbuild . extra_warnings (false) . warnings (false) . flag_if_supported ("-Wno-bitwise-op-parentheses") . flag_if_supported ("-Wno-literal-range") . flag_if_supported ("-Wno-parentheses") . flag_if_supported ("-Wno-shift-count-overflow") . flag_if_supported ("-Wno-shift-op-parentheses") . flag_if_supported ("-Wno-unused-but-set-variable") . flag_if_supported ("-std=c99") . flag_if_supported ("-ffreestanding") . flag_if_supported ("-nostdinc") . define ("_ALL_SOURCE" , "1") . define ("ROOT_INCLUDE_FEATURES" , Some (musl_dir . join ("include/features.h") . to_str () . unwrap ()) ,) . include (cfg . manifest_dir . join ("c_patches")) . include (musl_dir . join ("arch") . join (& cfg . musl_arch)) . include (musl_dir . join ("arch/generic")) . include (musl_dir . join ("src/include")) . include (musl_dir . join ("src/internal")) . include (obj_include) . include (musl_dir . join ("include")) . file (cfg . manifest_dir . join ("c_patches/alias.c")) ; for (sym_name , src_file) in source_map { cbuild . file (src_file) ; if let Some ((_names , syms)) = MULTIPLE_SYMBOLS . iter () . find (| (name , _syms) | * name == sym_name) { for sym in * syms { cbuild . define (sym , Some (format ! ("musl_{sym}") . as_str ())) ; } } else { cbuild . define (& sym_name , Some (format ! ("musl_{sym_name}") . as_str ())) ; } } if cfg ! (windows) { cbuild . compile (LIB_NAME) ; return ; } let objfiles = cbuild . compile_intermediates () ; let stat = cbuild . get_compiler () . to_command () . arg ("-r") . arg ("-o") . arg (& out_path) . args (objfiles) . status () . unwrap () ; assert ! (stat . success ()) ; println ! ("cargo::rustc-link-lib={LIB_NAME}") ; println ! ("cargo::rustc-link-search=native={}" , cfg . out_dir . display ()) ; validate_archive_symbols (& out_path) ; }
}

macro_rules! find_math_source_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_math_source in module {}", module_path!());
    };
}

mkfn!{
    find_math_source_introspect!();
    #[doc = " Build a map of `name -> path`. `name` is typically the symbol name, but this doesn't account"] #[doc = " for files that provide multiple symbols."] fn find_math_source (math_root : & Path , cfg : & Config) -> BTreeMap < String , PathBuf > { let mut map = BTreeMap :: new () ; let mut arch_dir = None ; for item in fs :: read_dir (math_root) . unwrap () { let path = item . unwrap () . path () ; let meta = fs :: metadata (& path) . unwrap () ; if meta . is_dir () { if path . file_name () . unwrap () == cfg . target_arch . as_str () { arch_dir = Some (path) ; } continue ; } if path . extension () . is_some_and (| ext | ext == "h") { continue ; } let sym_name = path . file_stem () . unwrap () ; map . insert (sym_name . to_str () . unwrap () . to_owned () , path . to_owned ()) ; } if let Some (arch_dir) = arch_dir { for item in fs :: read_dir (arch_dir) . unwrap () { let path = item . unwrap () . path () ; let sym_name = path . file_stem () . unwrap () ; if path . extension () . unwrap () == "s" { continue ; } map . insert (sym_name . to_str () . unwrap () . to_owned () , path) ; } } map }
}

macro_rules! validate_archive_symbols_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function validate_archive_symbols in module {}", module_path!());
    };
}

mkfn!{
    validate_archive_symbols_introspect!();
    #[doc = " Make sure we don't have something like a loose unprefixed `_cos` called somewhere, which could"] #[doc = " wind up linking to system libraries rather than the built musl library."] fn validate_archive_symbols (out_path : & Path) { const ALLOWED_UNDEF_PFX : & [& str] = & [".TOC" , "_GLOBAL_OFFSET_TABLE_" , "__x86.get_pc_thunk" , "__add" , "__aeabi_" , "__div" , "__eq" , "__extend" , "__fix" , "__float" , "__gcc_" , "__ge" , "__gt" , "__le" , "__lshr" , "__lt" , "__mul" , "__ne" , "__stack_chk_fail" , "__stack_chk_guard" , "__sub" , "__trunc" , "__undef" , "__bzero" , "bzero" , "feclearexcept" , "fegetround" , "feraiseexcept" , "fesetround" , "fetestexcept" ,] ; let out = Command :: new ("nm") . arg ("-guj") . arg (out_path) . stderr (Stdio :: inherit ()) . output () . unwrap () ; let undef = str :: from_utf8 (& out . stdout) . unwrap () ; let mut undef = undef . lines () . collect :: < Vec < _ > > () ; undef . retain (| sym | { ! ALLOWED_UNDEF_PFX . iter () . any (| pfx | sym . starts_with (pfx) || sym [1 ..] . starts_with (pfx)) }) ; assert ! (undef . is_empty () , "found disallowed undefined symbols: {undef:#?}") ; let out = Command :: new ("nm") . arg ("-gUj") . arg (out_path) . stderr (Stdio :: inherit ()) . output () . unwrap () ; let defined = str :: from_utf8 (& out . stdout) . unwrap () ; let mut defined = defined . lines () . collect :: < Vec < _ > > () ; defined . retain (| sym | { ! (sym . starts_with ("_musl_") || sym . starts_with ("musl_") || sym . starts_with ("__x86.get_pc_thunk")) }) ; assert ! (defined . is_empty () , "found unprefixed symbols: {defined:#?}") ; }
}