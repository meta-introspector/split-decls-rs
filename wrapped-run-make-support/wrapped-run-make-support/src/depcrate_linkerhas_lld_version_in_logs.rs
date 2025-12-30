// Generated macro for has_lld_version_in_logs (function)
macro_rules! Depcrate_linkerhas_lld_version_in_logs {
() => {
// Module: crate::linker
// Provides: {"has_lld_version_in_logs"}
// Dependencies: {}
fn has_lld_version_in_logs (stderr : & str) -> bool { let stderr = Regex :: new (r"warning: linker std(out|err):") . unwrap () . replace_all (& stderr , "") ; let lld_version_re = Regex :: new (r"^LLD [0-9]+\.[0-9]+\.[0-9]+") . unwrap () ; stderr . lines () . any (| line | lld_version_re . is_match (line . trim ())) }
};
}
