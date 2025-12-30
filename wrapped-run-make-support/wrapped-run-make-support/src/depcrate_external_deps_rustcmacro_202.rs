// Generated macro for macro_202 (macro)
macro_rules! Depcrate_external_deps_rustcmacro_202 {
() => {
// Module: crate::external_deps::rustc
// Provides: {"macro_202"}
// Dependencies: {}
crate :: macros :: impl_common_helpers ! (Rustc , | rustc : & mut Rustc | { if let Some (target) = & rustc . target { rustc . cmd . arg (& format ! ("--target={target}")) ; } }) ;
};
}
