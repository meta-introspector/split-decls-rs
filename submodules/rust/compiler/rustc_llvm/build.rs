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
mkuse!{use std :: env ;}
mkuse!{use std :: ffi :: { OsStr , OsString } ;}
mkuse!{use std :: fmt :: Display ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: process :: { Command , Stdio } ;}
mkitem!{const OPTIONAL_COMPONENTS : & [& str] = & ["x86" , "arm" , "aarch64" , "amdgpu" , "avr" , "loongarch" , "m68k" , "csky" , "mips" , "powerpc" , "systemz" , "jsbackend" , "webassembly" , "msp430" , "sparc" , "nvptx" , "hexagon" , "riscv" , "xtensa" , "bpf" ,] ;}
mkitem!{const REQUIRED_COMPONENTS : & [& str] = & ["ipo" , "bitreader" , "bitwriter" , "linker" , "asmparser" , "lto" , "coverage" , "instrumentation"] ;}

macro_rules! detect_llvm_link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_llvm_link in module {}", module_path!());
    };
}

mkfn!{
    detect_llvm_link_introspect!();
    fn detect_llvm_link () -> (& 'static str , & 'static str) { if tracked_env_var_os ("LLVM_LINK_SHARED") . is_some () { ("dylib" , "--link-shared") } else { ("static" , "--link-static") } }
}

macro_rules! restore_library_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function restore_library_path in module {}", module_path!());
    };
}

mkfn!{
    restore_library_path_introspect!();
    fn restore_library_path () { let key = tracked_env_var_os ("REAL_LIBRARY_PATH_VAR") . expect ("REAL_LIBRARY_PATH_VAR") ; if let Some (env) = tracked_env_var_os ("REAL_LIBRARY_PATH") { unsafe { env :: set_var (& key , env) ; } } else { unsafe { env :: remove_var (& key) ; } } }
}

macro_rules! tracked_env_var_os_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tracked_env_var_os in module {}", module_path!());
    };
}

mkfn!{
    tracked_env_var_os_introspect!();
    # [doc = " Reads an environment variable and adds it to dependencies."] # [doc = " Supposed to be used for all variables except those set for build scripts by cargo"] # [doc = " <https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts>"] fn tracked_env_var_os < K : AsRef < OsStr > + Display > (key : K) -> Option < OsString > { println ! ("cargo:rerun-if-env-changed={key}") ; env :: var_os (key) }
}

macro_rules! rerun_if_changed_anything_in_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rerun_if_changed_anything_in_dir in module {}", module_path!());
    };
}

mkfn!{
    rerun_if_changed_anything_in_dir_introspect!();
    fn rerun_if_changed_anything_in_dir (dir : & Path) { let mut stack = dir . read_dir () . unwrap () . map (| e | e . unwrap ()) . filter (| e | & * e . file_name () != ".git") . collect :: < Vec < _ > > () ; while let Some (entry) = stack . pop () { let path = entry . path () ; if entry . file_type () . unwrap () . is_dir () { stack . extend (path . read_dir () . unwrap () . map (| e | e . unwrap ())) ; } else { println ! ("cargo:rerun-if-changed={}" , path . display ()) ; } } }
}

macro_rules! output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function output in module {}", module_path!());
    };
}

mkfn!{
    output_introspect!();
    # [track_caller] fn output (cmd : & mut Command) -> String { let output = match cmd . stderr (Stdio :: inherit ()) . output () { Ok (status) => status , Err (e) => { println ! ("\n\nfailed to execute command: {cmd:?}\nerror: {e}\n\n") ; std :: process :: exit (1) ; } } ; if ! output . status . success () { panic ! ("command did not execute successfully: {:?}\n\
             expected success, got: {}" , cmd , output . status) ; } String :: from_utf8 (output . stdout) . unwrap () }
}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { if cfg ! (feature = "check_only") { return ; } for component in REQUIRED_COMPONENTS . iter () . chain (OPTIONAL_COMPONENTS . iter ()) { println ! ("cargo:rustc-check-cfg=cfg(llvm_component,values(\"{component}\"))") ; } if tracked_env_var_os ("RUST_CHECK") . is_some () { return ; } restore_library_path () ; let llvm_config = PathBuf :: from (tracked_env_var_os ("LLVM_CONFIG") . expect ("LLVM_CONFIG was not set")) ; println ! ("cargo:rerun-if-changed={}" , llvm_config . display ()) ; let target = env :: var ("TARGET") . expect ("TARGET was not set") ; let host = env :: var ("HOST") . expect ("HOST was not set") ; let is_crossed = target != host ; let components = output (Command :: new (& llvm_config) . arg ("--components")) ; let mut components = components . split_whitespace () . collect :: < Vec < _ > > () ; components . retain (| c | OPTIONAL_COMPONENTS . contains (c) || REQUIRED_COMPONENTS . contains (c)) ; for component in REQUIRED_COMPONENTS { if ! components . contains (component) { panic ! ("require llvm component {component} but wasn't found") ; } } for component in components . iter () { println ! ("cargo:rustc-cfg=llvm_component=\"{component}\"") ; } let mut cmd = Command :: new (& llvm_config) ; cmd . arg ("--cxxflags") ; let cxxflags = output (& mut cmd) ; let mut cfg = cc :: Build :: new () ; cfg . warnings (false) ; if std :: env :: var_os ("CI") . is_some () && ! target . contains ("msvc") { cfg . warnings_into_errors (true) ; } for flag in cxxflags . split_whitespace () { if is_crossed && flag . starts_with ("-m") { continue ; } if flag . starts_with ("-flto") { continue ; } if is_crossed && target . contains ("netbsd") && flag . contains ("date-time") { continue ; } if is_crossed && flag . starts_with ("-I") { cfg . flag (& flag . replace (& host , & target)) ; continue ; } cfg . flag (flag) ; } for component in & components { let mut flag = String :: from ("LLVM_COMPONENT_") ; flag . push_str (& component . to_uppercase ()) ; cfg . define (& flag , None) ; } if tracked_env_var_os ("LLVM_ENZYME") . is_some () { cfg . define ("ENZYME" , None) ; } if tracked_env_var_os ("LLVM_RUSTLLVM") . is_some () { cfg . define ("LLVM_RUSTLLVM" , None) ; } if tracked_env_var_os ("LLVM_ASSERTIONS") . is_none () { cfg . define ("NDEBUG" , None) ; } rerun_if_changed_anything_in_dir (Path :: new ("llvm-wrapper")) ; cfg . file ("llvm-wrapper/PassWrapper.cpp") . file ("llvm-wrapper/RustWrapper.cpp") . file ("llvm-wrapper/CoverageMappingWrapper.cpp") . file ("llvm-wrapper/SymbolWrapper.cpp") . file ("llvm-wrapper/Linker.cpp") . cpp (true) . cpp_link_stdlib (None) . compile ("llvm-wrapper") ; let (llvm_kind , llvm_link_arg) = detect_llvm_link () ; let mut cmd = Command :: new (& llvm_config) ; cmd . arg (llvm_link_arg) . arg ("--libs") ; if ! is_crossed || target . contains ("windows") && host . contains ("windows") { cmd . arg ("--system-libs") ; } if target . starts_with ("sparcv9") && target . contains ("solaris") { println ! ("cargo:rustc-link-lib=kstat") ; } if (target . starts_with ("arm") && ! target . starts_with ("arm64") && ! target . contains ("freebsd") && ! target . contains ("ohos")) || target . starts_with ("mips-") || target . starts_with ("mipsel-") || target . starts_with ("powerpc-") || target . starts_with ("sparc-") { println ! ("cargo:rustc-link-lib=atomic") ; } else if target . contains ("windows-gnu") { println ! ("cargo:rustc-link-lib=shell32") ; println ! ("cargo:rustc-link-lib=uuid") ; } else if target . contains ("haiku") || target . contains ("darwin") || (is_crossed && (target . contains ("dragonfly") || target . contains ("solaris"))) || target . contains ("cygwin") { println ! ("cargo:rustc-link-lib=z") ; } else if target . contains ("netbsd") { if target . starts_with ("i586") || target . starts_with ("i686") { println ! ("cargo:rustc-link-lib=atomic") ; } println ! ("cargo:rustc-link-lib=z") ; println ! ("cargo:rustc-link-lib=execinfo") ; } cmd . args (& components) ; for lib in output (& mut cmd) . split_whitespace () { let mut is_static = false ; let name = if let Some (stripped) = lib . strip_prefix ("-l") { stripped } else if let Some (stripped) = lib . strip_prefix ('-') { stripped } else if Path :: new (lib) . exists () { let path = Path :: new (lib) ; if lib . ends_with (".a") { is_static = true ; println ! ("cargo:rustc-link-search=native={}" , path . parent () . unwrap () . display ()) ; let name = path . file_stem () . unwrap () . to_str () . unwrap () ; name . trim_start_matches ("lib") } else { let name = path . file_name () . unwrap () . to_str () . unwrap () ; name . trim_end_matches (".lib") } } else if lib . ends_with (".lib") { lib . trim_end_matches (".lib") } else { continue ; } ; if name == "LLVMLineEditor" { continue ; } let kind = if name . starts_with ("LLVM") { llvm_kind } else if is_static { "static" } else { "dylib" } ; println ! ("cargo:rustc-link-lib={kind}={name}") ; } let mut cmd = Command :: new (& llvm_config) ; cmd . arg (llvm_link_arg) . arg ("--ldflags") ; for lib in output (& mut cmd) . split_whitespace () { if is_crossed { if let Some (stripped) = lib . strip_prefix ("-LIBPATH:") { println ! ("cargo:rustc-link-search=native={}" , stripped . replace (& host , & target)) ; } else if let Some (stripped) = lib . strip_prefix ("-L") { println ! ("cargo:rustc-link-search=native={}" , stripped . replace (& host , & target)) ; } } else if let Some (stripped) = lib . strip_prefix ("-LIBPATH:") { println ! ("cargo:rustc-link-search=native={stripped}") ; } else if let Some (stripped) = lib . strip_prefix ("-l") { println ! ("cargo:rustc-link-lib={stripped}") ; } else if let Some (stripped) = lib . strip_prefix ("-L") { println ! ("cargo:rustc-link-search=native={stripped}") ; } } let llvm_linker_flags = tracked_env_var_os ("LLVM_LINKER_FLAGS") ; if let Some (s) = llvm_linker_flags { for lib in s . into_string () . unwrap () . split_whitespace () { if let Some (stripped) = lib . strip_prefix ("-l") { println ! ("cargo:rustc-link-lib={stripped}") ; } else if let Some (stripped) = lib . strip_prefix ("-L") { println ! ("cargo:rustc-link-search=native={stripped}") ; } } } let llvm_static_stdcpp = tracked_env_var_os ("LLVM_STATIC_STDCPP") ; let llvm_use_libcxx = tracked_env_var_os ("LLVM_USE_LIBCXX") ; let stdcppname = if target . contains ("openbsd") { if target . contains ("sparc64") { "estdc++" } else { "c++" } } else if target . contains ("darwin") || target . contains ("freebsd") || target . contains ("windows-gnullvm") || target . contains ("aix") || target . contains ("ohos") { "c++" } else if target . contains ("netbsd") && llvm_static_stdcpp . is_some () { "stdc++_p" } else if llvm_use_libcxx . is_some () { "c++" } else { "stdc++" } ; if target . starts_with ("riscv") && ! target . contains ("freebsd") && ! target . contains ("openbsd") { println ! ("cargo:rustc-link-lib=atomic") ; } if ! target . contains ("msvc") { if let Some (s) = llvm_static_stdcpp { assert ! (! cxxflags . contains ("stdlib=libc++")) ; let path = PathBuf :: from (s) ; println ! ("cargo:rustc-link-search=native={}" , path . parent () . unwrap () . display ()) ; if target . contains ("windows") { println ! ("cargo:rustc-link-lib=static:-bundle={stdcppname}") ; } else { println ! ("cargo:rustc-link-lib=static={stdcppname}") ; } } else if cxxflags . contains ("stdlib=libc++") { println ! ("cargo:rustc-link-lib=c++") ; } else { println ! ("cargo:rustc-link-lib={stdcppname}") ; } } if target . contains ("aix") { println ! ("cargo:rustc-link-lib=c++abi") ; println ! ("cargo:rustc-link-lib=unwind") ; } if target . ends_with ("windows-gnu") { println ! ("cargo:rustc-link-lib=static:-bundle=pthread") ; } }
}