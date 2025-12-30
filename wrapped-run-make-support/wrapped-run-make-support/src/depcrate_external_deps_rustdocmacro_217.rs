// Generated macro for macro_217 (macro)
macro_rules! Depcrate_external_deps_rustdocmacro_217 {
() => {
// Module: crate::external_deps::rustdoc
// Provides: {"macro_217"}
// Dependencies: {}
crate :: macros :: impl_common_helpers ! (Rustdoc , | rustdoc : & mut Rustdoc | { if let Some (target) = & rustdoc . target { rustdoc . cmd . arg (& format ! ("--target={target}")) ; } }) ;
};
}
