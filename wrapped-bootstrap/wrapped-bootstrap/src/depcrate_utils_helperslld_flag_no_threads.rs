// Generated macro for lld_flag_no_threads (function)
macro_rules! Depcrate_utils_helperslld_flag_no_threads {
() => {
// Module: crate::utils::helpers
// Provides: {"lld_flag_no_threads"}
// Dependencies: {}
# [doc = " Returns a flag that configures LLD to use only a single thread."] # [doc = " If we use an external LLD, we need to find out which version is it to know which flag should we"] # [doc = " pass to it (LLD older than version 10 had a different flag)."] fn lld_flag_no_threads (builder : & Builder < '_ > , lld_mode : LldMode , is_windows : bool) -> & 'static str { static LLD_NO_THREADS : OnceLock < (& 'static str , & 'static str) > = OnceLock :: new () ; let new_flags = ("/threads:1" , "--threads=1") ; let old_flags = ("/no-threads" , "--no-threads") ; let (windows_flag , other_flag) = LLD_NO_THREADS . get_or_init (| | { let newer_version = match lld_mode { LldMode :: External => { let mut cmd = command ("lld") ; cmd . arg ("-flavor") . arg ("ld") . arg ("--version") ; let out = cmd . run_capture_stdout (builder) . stdout () ; match (out . find (char :: is_numeric) , out . find ('.')) { (Some (b) , Some (e)) => out . as_str () [b .. e] . parse :: < i32 > () . ok () . unwrap_or (14) > 10 , _ => true , } } _ => true , } ; if newer_version { new_flags } else { old_flags } }) ; if is_windows { windows_flag } else { other_flag } }
};
}
