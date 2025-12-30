// Generated macro for clone_fields (macro)
macro_rules! Depcrate_impl_macrosclone_fields {
() => {
// Module: crate::impl_macros
// Provides: {"clone_fields"}
// Dependencies: {}
macro_rules ! clone_fields { ($ ($ field : ident) ,*) => { # [inline] fn clone (& self) -> Self { Self { $ ($ field : self .$ field . clone () ,) * } } } }
};
}
