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