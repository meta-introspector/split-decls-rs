// Generated macro for impl_260 (impl)
macro_rules! Depcrateimpl_260 {
() => {
// Module: crate
// Provides: {"impl_260"}
// Dependencies: {}
impl RustcInfo { fn new () -> RustcInfo { let output = ProcessBuilder :: new ("rustc") . arg ("-vV") . exec_with_output () . expect ("rustc should exec") ; let verbose_version = String :: from_utf8 (output . stdout) . expect ("utf8 output") ; let host = verbose_version . lines () . filter_map (| line | line . strip_prefix ("host: ")) . next () . expect ("verbose version has host: field") . to_string () ; RustcInfo { verbose_version , host , } } }
};
}
