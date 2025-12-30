// Generated macro for impl_26 (impl)
macro_rules! Depcrate_chainimpl_26 {
() => {
// Module: crate::chain
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl DoubleEndedIterator for Chain < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { match & mut self . state { Linked { mut next } => { let mut rest = Vec :: new () ; while let Some (cause) = next { next = cause . source () ; rest . push (cause) ; } let mut rest = rest . into_iter () ; let last = rest . next_back () ; self . state = Buffered { rest } ; last } Buffered { rest } => rest . next_back () , } } }
};
}
