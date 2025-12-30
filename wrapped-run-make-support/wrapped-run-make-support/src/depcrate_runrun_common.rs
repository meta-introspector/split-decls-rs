// Generated macro for run_common (function)
macro_rules! Depcrate_runrun_common {
() => {
// Module: crate::run
// Provides: {"run_common"}
// Dependencies: {}
# [track_caller] fn run_common (name : & str , args : Option < & [& str] >) -> Command { let mut bin_path = PathBuf :: new () ; bin_path . push (cwd ()) ; bin_path . push (name) ; let ld_lib_path_envvar = env_var ("LD_LIB_PATH_ENVVAR") ; let mut cmd = if let Some (rtc) = env :: var_os ("REMOTE_TEST_CLIENT") { let mut cmd = Command :: new (rtc) ; cmd . arg ("run") ; cmd . arg ("0") ; cmd . arg (bin_path) ; cmd } else if let Ok (runner) = std :: env :: var ("RUNNER") { let mut args = split_maybe_args (& runner) ; let prog = args . remove (0) ; let mut cmd = Command :: new (prog) ; for arg in args { cmd . arg (arg) ; } cmd . arg ("--") ; cmd . arg (bin_path) ; cmd } else { Command :: new (bin_path) } ; if let Some (args) = args { for arg in args { cmd . arg (arg) ; } } cmd . env (& ld_lib_path_envvar , { let mut paths = vec ! [] ; paths . push (cwd ()) ; for p in env :: split_paths (& env_var ("TARGET_EXE_DYLIB_PATH")) { paths . push (p . to_path_buf ()) ; } for p in env :: split_paths (& env_var (& ld_lib_path_envvar)) { paths . push (p . to_path_buf ()) ; } env :: join_paths (paths . iter ()) . unwrap () }) ; cmd . env ("LC_ALL" , "C") ; cmd }
};
}
