// Generated macro for impl_83 (impl)
macro_rules! Depcrateimpl_83 {
() => {
// Module: crate
// Provides: {"impl_83"}
// Dependencies: {}
impl PopError { # [doc = " Returns `true` if the queue is empty but not closed."] pub fn is_empty (& self) -> bool { match self { PopError :: Empty => true , PopError :: Closed => false , } } # [doc = " Returns `true` if the queue is empty and closed."] pub fn is_closed (& self) -> bool { match self { PopError :: Empty => false , PopError :: Closed => true , } } }
};
}
