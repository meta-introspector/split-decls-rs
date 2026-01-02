mkuse!{use std :: env ;}
mkuse!{use std :: path :: PathBuf ;}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { if let Ok (rt) = tracked_env_var ("LLVM_PROFILER_RT_LIB") { let rt = PathBuf :: from (rt) ; if let Some (lib) = rt . file_name () { if let Some (dir) = rt . parent () { println ! ("cargo::rustc-link-search=native={}" , dir . display ()) ; } println ! ("cargo::rustc-link-lib=static:+verbatim={}" , lib . to_str () . unwrap ()) ; return ; } } let target_os = env :: var ("CARGO_CFG_TARGET_OS") . expect ("CARGO_CFG_TARGET_OS was not set") ; let target_env = env :: var ("CARGO_CFG_TARGET_ENV") . expect ("CARGO_CFG_TARGET_ENV was not set") ; let cfg = & mut cc :: Build :: new () ; let profile_sources = vec ! ["GCDAProfiling.c" , "InstrProfiling.c" , "InstrProfilingBuffer.c" , "InstrProfilingFile.c" , "InstrProfilingInternal.c" , "InstrProfilingMerge.c" , "InstrProfilingMergeFile.c" , "InstrProfilingNameVar.c" , "InstrProfilingPlatformAIX.c" , "InstrProfilingPlatformDarwin.c" , "InstrProfilingPlatformFuchsia.c" , "InstrProfilingPlatformLinux.c" , "InstrProfilingPlatformOther.c" , "InstrProfilingPlatformWindows.c" , "InstrProfilingRuntime.cpp" , "InstrProfilingUtil.c" , "InstrProfilingValue.c" , "InstrProfilingVersionVar.c" , "InstrProfilingWriter.c" , "WindowsMMap.c" ,] ; if target_env == "msvc" { cfg . flag ("/Zl") ; cfg . define ("strdup" , Some ("_strdup")) ; cfg . define ("open" , Some ("_open")) ; cfg . define ("fdopen" , Some ("_fdopen")) ; cfg . define ("getpid" , Some ("_getpid")) ; cfg . define ("fileno" , Some ("_fileno")) ; } else { cfg . flag ("-fno-builtin") ; cfg . flag ("-fomit-frame-pointer") ; cfg . define ("VISIBILITY_HIDDEN" , None) ; if target_os != "windows" { cfg . flag ("-fvisibility=hidden") ; cfg . define ("COMPILER_RT_HAS_UNAME" , Some ("1")) ; } } if env :: var_os ("CARGO_CFG_UNIX") . is_some () { cfg . define ("COMPILER_RT_HAS_FCNTL_LCK" , Some ("1")) ; } if env :: var_os ("CARGO_CFG_TARGET_HAS_ATOMIC") . map (| features | features . to_string_lossy () . to_lowercase () . contains ("ptr")) . unwrap_or (false) { cfg . define ("COMPILER_RT_HAS_ATOMICS" , Some ("1")) ; } let root = PathBuf :: from (tracked_env_var_or_fallback ("RUST_COMPILER_RT_FOR_PROFILER" , "../../src/llvm-project/compiler-rt" ,)) ; let src_root = root . join ("lib") . join ("profile") ; assert ! (src_root . exists () , "profiler runtime source directory not found: {src_root:?}") ; println ! ("cargo::rerun-if-changed={}" , src_root . display ()) ; for file in profile_sources { cfg . file (src_root . join (file)) ; } let include = root . join ("include") ; println ! ("cargo::rerun-if-changed={}" , include . display ()) ; cfg . include (include) ; cfg . warnings (false) ; cfg . compile ("profiler-rt") ; }
}

macro_rules! tracked_env_var_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tracked_env_var in module {}", module_path!());
    };
}

mkfn!{
    tracked_env_var_introspect!();
    fn tracked_env_var (key : & str) -> Result < String , env :: VarError > { println ! ("cargo::rerun-if-env-changed={key}") ; env :: var (key) }
}

macro_rules! tracked_env_var_or_fallback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tracked_env_var_or_fallback in module {}", module_path!());
    };
}

mkfn!{
    tracked_env_var_or_fallback_introspect!();
    fn tracked_env_var_or_fallback (key : & str , fallback : & str) -> String { tracked_env_var (key) . unwrap_or_else (| _ | { println ! ("cargo::warning={key} was not set; falling back to {fallback:?}") ; fallback . to_owned () }) }
}