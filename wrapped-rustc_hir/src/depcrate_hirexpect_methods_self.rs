// Generated macro for expect_methods_self (macro)
macro_rules! Depcrate_hirexpect_methods_self {
() => {
// Module: crate::hir
// Provides: {"expect_methods_self"}
// Dependencies: {}
macro_rules ! expect_methods_self { ($ ($ name : ident , $ ret_ty : ty , $ pat : pat , $ ret_val : expr ;) *) => { $ (# [track_caller] pub fn $ name (& self) -> $ ret_ty { let $ pat = self else { expect_failed (stringify ! ($ ident) , self) } ; $ ret_val }) * } }
};
}
