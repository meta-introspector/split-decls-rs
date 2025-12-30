// Generated macro for foreach_cas (macro)
macro_rules! Depcrate_aarch64_linuxforeach_cas {
() => {
// Module: crate::aarch64_linux
// Provides: {"foreach_cas"}
// Dependencies: {}
# [doc = " Generate different macros for cas/swp/add/clr/eor/set so that we can test them separately."] # [macro_export] macro_rules ! foreach_cas { ($ macro : path) => { foreach_bytes ! ($ macro , cas) ; } ; }
};
}
