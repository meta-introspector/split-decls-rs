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
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: ffi :: OsStr ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: { env as std_env , fs } ;}
mkuse!{use boml :: Toml ;}
mkuse!{use boml :: types :: TomlValue ;}
mkuse!{use crate :: utils :: { create_dir , create_symlink , get_os_name , get_sysroot_dir , run_command_with_output , rustc_version_info , split_args , } ;}
mkitem!{mkenum!{#[derive (Default , PartialEq , Eq , Clone , Copy , Debug)] pub enum Channel { #[default] Debug , Release , }}}
mkitem!{mkimpl!{impl Channel { pub fn as_str (self) -> & 'static str { match self { Self :: Debug => "debug" , Self :: Release => "release" , } } }}}

macro_rules! failed_config_parsing_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function failed_config_parsing in module {}", module_path!());
    };
}

mkfn!{
    failed_config_parsing_introspect!();
    fn failed_config_parsing (config_file : & Path , err : & str) -> Result < ConfigFile , String > { Err (format ! ("Failed to parse `{}`: {}" , config_file . display () , err)) }
}
mkitem!{mkstruct!{#[derive (Default)] pub struct ConfigFile { gcc_path : Option < String > , download_gccjit : Option < bool > , }}}
mkitem!{mkimpl!{impl ConfigFile { pub fn new (config_file : & Path) -> Result < Self , String > { let content = fs :: read_to_string (config_file) . map_err (| _ | { format ! ("Failed to read `{}`. Take a look at `Readme.md` to see how to set up the project" , config_file . display () ,) }) ? ; let toml = Toml :: parse (& content) . map_err (| err | { format ! ("Error occurred around `{}`: {:?}" , & content [err . start ..= err . end] , err . kind) }) ? ; let mut config = Self :: default () ; for (key , value) in toml . iter () { match (key , value) { ("gcc-path" , TomlValue :: String (value)) => { config . gcc_path = Some (value . as_str () . to_string ()) } ("gcc-path" , _) => { return failed_config_parsing (config_file , "Expected a string for `gcc-path`") ; } ("download-gccjit" , TomlValue :: Boolean (value)) => { config . download_gccjit = Some (* value) } ("download-gccjit" , _) => { return failed_config_parsing (config_file , "Expected a boolean for `download-gccjit`" ,) ; } _ => return failed_config_parsing (config_file , & format ! ("Unknown key `{key}`")) , } } match (config . gcc_path . as_mut () , config . download_gccjit) { (None , None | Some (false)) => { return failed_config_parsing (config_file , "At least one of `gcc-path` or `download-gccjit` value must be set" ,) ; } (Some (_) , Some (true)) => { println ! ("WARNING: both `gcc-path` and `download-gccjit` arguments are used, \
                    ignoring `gcc-path`") ; } (Some (gcc_path) , _) => { let path = Path :: new (gcc_path) ; * gcc_path = path . canonicalize () . map_err (| err | format ! ("Failed to get absolute path of `{gcc_path}`: {err:?}")) ? . display () . to_string () ; } _ => { } } Ok (config) } }}}
mkitem!{mkstruct!{#[derive (Default , Debug , Clone)] pub struct ConfigInfo { pub target : String , pub target_triple : String , pub host_triple : String , pub rustc_command : Vec < String > , pub run_in_vm : bool , pub cargo_target_dir : String , pub dylib_ext : String , pub sysroot_release_channel : bool , pub channel : Channel , pub sysroot_panic_abort : bool , pub cg_backend_path : String , pub sysroot_path : String , pub gcc_path : Option < String > , config_file : Option < String > , cg_gcc_path : Option < PathBuf > , pub no_download : bool , pub no_default_features : bool , pub backend : Option < String > , pub features : Vec < String > , }}}
mkitem!{mkimpl!{impl ConfigInfo { #[doc = " Returns `true` if the argument was taken into account."] pub fn parse_argument (& mut self , arg : & str , args : & mut impl Iterator < Item = String > ,) -> Result < bool , String > { match arg { "--features" => { if let Some (arg) = args . next () { self . features . push (arg) ; } else { return Err ("Expected a value after `--features`, found nothing" . to_string ()) ; } } "--target" => { if let Some (arg) = args . next () { self . target = arg ; } else { return Err ("Expected a value after `--target`, found nothing" . to_string ()) ; } } "--target-triple" => match args . next () { Some (arg) if ! arg . is_empty () => self . target_triple = arg . to_string () , _ => { return Err ("Expected a value after `--target-triple`, found nothing" . to_string ()) ; } } , "--out-dir" => match args . next () { Some (arg) if ! arg . is_empty () => { self . cargo_target_dir = arg . to_string () ; } _ => return Err ("Expected a value after `--out-dir`, found nothing" . to_string ()) , } , "--config-file" => match args . next () { Some (arg) if ! arg . is_empty () => { self . config_file = Some (arg . to_string ()) ; } _ => { return Err ("Expected a value after `--config-file`, found nothing" . to_string ()) ; } } , "--release-sysroot" => self . sysroot_release_channel = true , "--release" => self . channel = Channel :: Release , "--sysroot-panic-abort" => self . sysroot_panic_abort = true , "--gcc-path" => match args . next () { Some (arg) if ! arg . is_empty () => { self . gcc_path = Some (arg) ; } _ => { return Err ("Expected a value after `--gcc-path`, found nothing" . to_string ()) ; } } , "--cg_gcc-path" => match args . next () { Some (arg) if ! arg . is_empty () => { self . cg_gcc_path = Some (arg . into ()) ; } _ => { return Err ("Expected a value after `--cg_gcc-path`, found nothing" . to_string ()) ; } } , "--use-backend" => match args . next () { Some (backend) if ! backend . is_empty () => self . backend = Some (backend) , _ => return Err ("Expected an argument after `--use-backend`, found nothing" . into ()) , } , "--no-default-features" => self . no_default_features = true , _ => return Ok (false) , } Ok (true) } pub fn rustc_command_vec (& self) -> Vec < & dyn AsRef < OsStr > > { let mut command : Vec < & dyn AsRef < OsStr > > = Vec :: with_capacity (self . rustc_command . len ()) ; for arg in self . rustc_command . iter () { command . push (arg) ; } command } pub fn get_gcc_commit (& self) -> Result < String , String > { let commit_hash_file = self . compute_path ("libgccjit.version") ; let content = fs :: read_to_string (& commit_hash_file) . map_err (| _ | { format ! ("Failed to read `{}`. Take a look at `Readme.md` to see how to set up the project" , commit_hash_file . display () ,) }) ? ; let commit = content . trim () ; if commit . contains ('/') || commit . contains ('\\') { return Err (format ! ("{}: invalid commit hash `{}`" , commit_hash_file . display () , commit ,)) ; } Ok (commit . to_string ()) } fn download_gccjit_if_needed (& mut self) -> Result < () , String > { let output_dir = Path :: new (crate :: BUILD_DIR) . join ("libgccjit") ; let commit = self . get_gcc_commit () ? ; let output_dir = output_dir . join (& commit) ; if ! output_dir . is_dir () { create_dir (& output_dir) ? ; } let output_dir = output_dir . canonicalize () . map_err (| err | { format ! ("Failed to get absolute path of `{}`: {:?}" , output_dir . display () , err) }) ? ; let libgccjit_so_name = "libgccjit.so" ; let libgccjit_so = output_dir . join (libgccjit_so_name) ; if ! libgccjit_so . is_file () && ! self . no_download { let tempfile_name = format ! ("{libgccjit_so_name}.download") ; let tempfile = output_dir . join (& tempfile_name) ; let is_in_ci = std :: env :: var ("GITHUB_ACTIONS") . is_ok () ; download_gccjit (& commit , & output_dir , tempfile_name , ! is_in_ci) ? ; let libgccjit_so = output_dir . join (libgccjit_so_name) ; std :: fs :: rename (& tempfile , & libgccjit_so) . map_err (| err | { format ! ("Failed to rename `{}` into `{}`: {:?}" , tempfile . display () , libgccjit_so . display () , err ,) }) ? ; println ! ("Downloaded libgccjit.so version {commit} successfully!") ; create_symlink (& libgccjit_so , output_dir . join (format ! ("{libgccjit_so_name}.0"))) ? ; } let gcc_path = output_dir . display () . to_string () ; println ! ("Using `{gcc_path}` as path for libgccjit") ; self . gcc_path = Some (gcc_path) ; Ok (()) } pub fn compute_path < P : AsRef < Path > > (& self , other : P) -> PathBuf { match self . cg_gcc_path { Some (ref path) => path . join (other) , None => PathBuf :: new () . join (other) , } } pub fn setup_gcc_path (& mut self) -> Result < () , String > { if let Some (gcc_path) = & self . gcc_path { println ! ("`--gcc-path` was provided, ignoring config file. Using `{gcc_path}` as path for libgccjit") ; return Ok (()) ; } let config_file = match self . config_file . as_deref () { Some (config_file) => config_file . into () , None => self . compute_path ("config.toml") , } ; let ConfigFile { gcc_path , download_gccjit } = ConfigFile :: new (& config_file) ? ; if let Some (true) = download_gccjit { self . download_gccjit_if_needed () ? ; return Ok (()) ; } let Some (gcc_path) = gcc_path else { return Err (format ! ("missing `gcc-path` value from `{}`" , config_file . display ())) ; } ; println ! ("GCC path retrieved from `{}`. Using `{}` as path for libgccjit" , config_file . display () , gcc_path) ; self . gcc_path = Some (gcc_path) ; Ok (()) } pub fn setup (& mut self , env : & mut HashMap < String , String > , use_system_gcc : bool ,) -> Result < () , String > { env . insert ("CARGO_INCREMENTAL" . to_string () , "0" . to_string ()) ; let gcc_path = if ! use_system_gcc { if self . gcc_path . is_none () { self . setup_gcc_path () ? ; } self . gcc_path . clone () . expect ("The config module should have emitted an error if the GCC path wasn't provided" ,) } else { String :: new () } ; env . insert ("GCC_PATH" . to_string () , gcc_path . clone ()) ; if self . cargo_target_dir . is_empty () { match env . get ("CARGO_TARGET_DIR") . filter (| dir | ! dir . is_empty ()) { Some (cargo_target_dir) => self . cargo_target_dir = cargo_target_dir . clone () , None => self . cargo_target_dir = "target/out" . to_string () , } } let os_name = get_os_name () ? ; self . dylib_ext = match os_name . as_str () { "Linux" => "so" , "Darwin" => "dylib" , os => return Err (format ! ("unsupported OS `{os}`")) , } . to_string () ; let rustc = match env . get ("RUSTC") { Some (r) if ! r . is_empty () => r . to_string () , _ => "rustc" . to_string () , } ; self . host_triple = match rustc_version_info (Some (& rustc)) ? . host { Some (host) => host , None => return Err ("no host found" . to_string ()) , } ; if self . target_triple . is_empty () { self . target_triple = self . host_triple . clone () ; } if self . target . is_empty () && ! self . target_triple . is_empty () { self . target = self . target_triple . clone () ; } let mut linker = None ; if self . host_triple != self . target_triple { if self . target_triple . is_empty () { return Err ("Unknown non-native platform" . to_string ()) ; } linker = Some (format ! ("-Clinker={}-gcc" , self . target_triple)) ; self . run_in_vm = true ; } let current_dir = std_env :: current_dir () . map_err (| error | format ! ("`current_dir` failed: {error:?}")) ? ; let channel = if self . channel == Channel :: Release { "release" } else if let Some (channel) = env . get ("CHANNEL") { channel . as_str () } else { "debug" } ; let mut rustflags = Vec :: new () ; self . cg_backend_path = current_dir . join ("target") . join (channel) . join (format ! ("librustc_codegen_gcc.{}" , self . dylib_ext)) . display () . to_string () ; self . sysroot_path = current_dir . join (get_sysroot_dir ()) . join ("sysroot") . display () . to_string () ; if let Some (backend) = & self . backend { rustflags . push (format ! ("-Zcodegen-backend={backend}")) ; } else { rustflags . extend_from_slice (& ["--sysroot" . to_string () , self . sysroot_path . clone () , format ! ("-Zcodegen-backend={}" , self . cg_backend_path) ,]) ; } if let Some (cg_rustflags) = env . get ("CG_RUSTFLAGS") { rustflags . extend_from_slice (& split_args (cg_rustflags) ?) ; } if let Some (test_flags) = env . get ("TEST_FLAGS") { rustflags . extend_from_slice (& split_args (test_flags) ?) ; } if let Some (linker) = linker { rustflags . push (linker . to_string ()) ; } if self . no_default_features { rustflags . push ("-Csymbol-mangling-version=v0" . to_string ()) ; } if os_name == "Darwin" { rustflags . extend_from_slice (& ["-Clink-arg=-undefined" . to_string () , "-Clink-arg=dynamic_lookup" . to_string () ,]) ; } env . insert ("RUSTFLAGS" . to_string () , rustflags . join (" ")) ; env . insert ("RUSTC_LOG" . to_string () , "warn" . to_string ()) ; let sysroot = current_dir . join (get_sysroot_dir ()) . join (format ! ("sysroot/lib/rustlib/{}/lib" , self . target_triple)) ; let ld_library_path = format ! ("{target}:{sysroot}:{gcc_path}" , target = self . cargo_target_dir , sysroot = sysroot . display () , gcc_path = gcc_path ,) ; env . insert ("LIBRARY_PATH" . to_string () , ld_library_path . clone ()) ; env . insert ("LD_LIBRARY_PATH" . to_string () , ld_library_path . clone ()) ; env . insert ("DYLD_LIBRARY_PATH" . to_string () , ld_library_path) ; let path = std :: env :: var ("PATH") . unwrap_or_default () ; env . insert ("PATH" . to_string () , format ! ("/opt/gcc/bin:/opt/m68k-unknown-linux-gnu/bin{}{}" , if path . is_empty () { "" } else { ":" } , path) ,) ; self . rustc_command = vec ! [rustc] ; self . rustc_command . extend_from_slice (& rustflags) ; self . rustc_command . extend_from_slice (& ["-L" . to_string () , format ! ("crate={}" , self . cargo_target_dir) , "--out-dir" . to_string () , self . cargo_target_dir . clone () ,]) ; if ! env . contains_key ("RUSTC_LOG") { env . insert ("RUSTC_LOG" . to_string () , "warn" . to_string ()) ; } Ok (()) } pub fn show_usage () { println ! ("\
    --features [arg]       : Add a new feature [arg]
    --target-triple [arg]  : Set the target triple to [arg]
    --target [arg]         : Set the target to [arg]
    --out-dir              : Location where the files will be generated
    --release              : Build in release mode
    --release-sysroot      : Build sysroot in release mode
    --sysroot-panic-abort  : Build the sysroot without unwinding support
    --config-file          : Location of the config file to be used
    --gcc-path             : Location of the GCC root folder
    --cg_gcc-path          : Location of the rustc_codegen_gcc root folder (used
                             when ran from another directory)
    --no-default-features  : Add `--no-default-features` flag to cargo commands
    --use-backend          : Useful only for rustc testsuite") ; } }}}

macro_rules! download_gccjit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function download_gccjit in module {}", module_path!());
    };
}

mkfn!{
    download_gccjit_introspect!();
    fn download_gccjit (commit : & str , output_dir : & Path , tempfile_name : String , with_progress_bar : bool ,) -> Result < () , String > { let url = if std :: env :: consts :: OS == "linux" && std :: env :: consts :: ARCH == "x86_64" { format ! ("https://github.com/rust-lang/gcc/releases/download/master-{commit}/libgccjit.so") } else { eprintln ! ("\
Pre-compiled libgccjit.so not available for this os or architecture.
Please compile it yourself and update the `config.toml` file
to `download-gccjit = false` and set `gcc-path` to the appropriate directory.") ; return Err (String :: from ("no appropriate pre-compiled libgccjit.so available for download" ,)) ; } ; println ! ("Downloading `{url}`...") ; let mut ret = run_command_with_output (& [& "curl" , & "--speed-time" , & "30" , & "--speed-limit" , & "10" , & "--connect-timeout" , & "30" , & "-o" , & tempfile_name , & "--retry" , & "3" , & "-SRfL" , if with_progress_bar { & "--progress-bar" } else { & "-s" } , & url . as_str () ,] , Some (output_dir) ,) ; if ret . is_err () && cfg ! (windows) { eprintln ! ("Fallback to PowerShell") ; ret = run_command_with_output (& [& "PowerShell.exe" , & "/nologo" , & "-Command" , & "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;" , & format ! ("(New-Object System.Net.WebClient).DownloadFile('{url}', '{tempfile_name}')" ,) . as_str () ,] , Some (output_dir) ,) ; } ret }
}