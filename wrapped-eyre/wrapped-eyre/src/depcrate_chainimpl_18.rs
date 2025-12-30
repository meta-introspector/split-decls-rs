// Generated macro for impl_18 (impl)
macro_rules! Depcrate_chainimpl_18 {
() => {
// Module: crate::chain
// Provides: {"impl_18"}
// Dependencies: {}
impl ExactSizeIterator for Chain < '_ > { fn len (& self) -> usize { match & self . state { Linked { mut next } => { let mut len = 0 ; while let Some (cause) = next { next = cause . source () ; len += 1 ; } len } Buffered { rest } => rest . len () , } } }
};
}
