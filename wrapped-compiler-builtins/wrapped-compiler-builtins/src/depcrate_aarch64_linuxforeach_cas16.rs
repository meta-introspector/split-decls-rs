// Generated macro for foreach_cas16 (macro)
macro_rules! Depcrate_aarch64_linuxforeach_cas16 {
() => {
// Module: crate::aarch64_linux
// Provides: {"foreach_cas16"}
// Dependencies: {}
# [doc = " Only CAS supports 16 bytes, and it has a different implementation that uses a different macro."] # [macro_export] macro_rules ! foreach_cas16 { ($ macro : path) => { foreach_ordering ! ($ macro , __aarch64_cas16) ; } ; }
};
}
