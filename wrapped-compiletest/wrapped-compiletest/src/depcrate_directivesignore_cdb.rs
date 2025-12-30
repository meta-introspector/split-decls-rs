// Generated macro for ignore_cdb (function)
macro_rules! Depcrate_directivesignore_cdb {
() => {
// Module: crate::directives
// Provides: {"ignore_cdb"}
// Dependencies: {}
fn ignore_cdb (config : & Config , line : & str) -> IgnoreDecision { if config . debugger != Some (Debugger :: Cdb) { return IgnoreDecision :: Continue ; } if let Some (actual_version) = config . cdb_version { if let Some (rest) = line . strip_prefix ("min-cdb-version:") . map (str :: trim) { let min_version = extract_cdb_version (rest) . unwrap_or_else (| | { panic ! ("couldn't parse version range: {:?}" , rest) ; }) ; if actual_version < min_version { return IgnoreDecision :: Ignore { reason : format ! ("ignored when the CDB version is lower than {rest}") , } ; } } } IgnoreDecision :: Continue }
};
}
