// Generated macro for impl_25 (impl)
macro_rules! Depcrate_chainimpl_25 {
() => {
// Module: crate::chain
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > Iterator for Chain < 'a > { type Item = & 'a (dyn StdError + 'static) ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . state { Linked { next } => { let error = (* next) ? ; * next = error . source () ; Some (error) } # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] Buffered { rest } => rest . next () , } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
