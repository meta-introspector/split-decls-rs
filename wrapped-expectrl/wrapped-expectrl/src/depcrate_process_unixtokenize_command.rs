// Generated macro for tokenize_command (function)
macro_rules! Depcrate_process_unixtokenize_command {
() => {
// Module: crate::process::unix
// Provides: {"tokenize_command"}
// Dependencies: {}
# [doc = " Turn e.g. \"prog arg1 arg2\" into [\"prog\", \"arg1\", \"arg2\"]"] # [doc = " It takes care of single and double quotes but,"] # [doc = ""] # [doc = " It doesn't cover all edge cases."] # [doc = " So it may not be compatible with real shell arguments parsing."] fn tokenize_command (program : & str) -> Vec < String > { let re = regex :: Regex :: new (r#""[^"]+"|'[^']+'|[^'" ]+"#) . unwrap () ; let mut res = vec ! [] ; for cap in re . captures_iter (program) { res . push (cap [0] . to_string ()) ; } res }
};
}
