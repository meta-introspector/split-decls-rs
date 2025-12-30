// Generated macro for ignore_lldb (function)
macro_rules! Depcrate_directivesignore_lldb {
() => {
// Module: crate::directives
// Provides: {"ignore_lldb"}
// Dependencies: {}
fn ignore_lldb (config : & Config , line : & str) -> IgnoreDecision { if config . debugger != Some (Debugger :: Lldb) { return IgnoreDecision :: Continue ; } if let Some (actual_version) = config . lldb_version { if let Some (rest) = line . strip_prefix ("min-lldb-version:") . map (str :: trim) { let min_version = rest . parse () . unwrap_or_else (| e | { panic ! ("Unexpected format of LLDB version string: {}\n{:?}" , rest , e) ; }) ; if actual_version < min_version { return IgnoreDecision :: Ignore { reason : format ! ("ignored when the LLDB version is {rest}") , } ; } } } IgnoreDecision :: Continue }
};
}
