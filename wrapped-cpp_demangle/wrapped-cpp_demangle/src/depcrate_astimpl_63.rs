// Generated macro for impl_63 (impl)
macro_rules! Depcrate_astimpl_63 {
() => {
// Module: crate::ast
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'ctx , 'a , W > Drop for AutoDemangleContextInnerBarrier < 'ctx , 'a , W > where W : 'a + DemangleWrite , 'a : 'ctx , { fn drop (& mut self) { if ! self . ctx . inner . is_empty () { log ! ("Context inner was not emptied, did demangling fail?") ; } mem :: swap (& mut self . saved_inner , & mut self . ctx . inner) ; } }
};
}
