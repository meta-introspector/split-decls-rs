// Generated macro for impl_43 (impl)
macro_rules! Depcrate_interruptimpl_43 {
() => {
// Module: crate::interrupt
// Provides: {"impl_43"}
// Dependencies: {}
impl < I , EFN , E > Iterator for Iter < I , EFN > where I : Iterator , EFN : FnOnce () -> E , { type Item = Result < I :: Item , E > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } }
};
}
