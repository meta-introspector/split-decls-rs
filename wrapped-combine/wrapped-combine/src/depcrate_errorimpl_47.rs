// Generated macro for impl_47 (impl)
macro_rules! Depcrate_errorimpl_47 {
() => {
// Module: crate::error
// Provides: {"impl_47"}
// Dependencies: {}
impl < T > AsMut < T > for Commit < T > { fn as_mut (& mut self) -> & mut T { match * self { Commit :: Peek (ref mut t) | Commit :: Commit (ref mut t) => t , } } }
};
}
