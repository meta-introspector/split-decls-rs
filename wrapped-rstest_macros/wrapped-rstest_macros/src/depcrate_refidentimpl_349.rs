// Generated macro for impl_349 (impl)
macro_rules! Depcrate_refidentimpl_349 {
() => {
// Module: crate::refident
// Provides: {"impl_349"}
// Dependencies: {}
impl RemoveMutability for FnArg { fn remove_mutability (& mut self) { if let FnArg :: Typed (PatType { pat , .. }) = self { if let Pat :: Ident (ident) = pat . as_mut () { ident . mutability = None } } ; } }
};
}
