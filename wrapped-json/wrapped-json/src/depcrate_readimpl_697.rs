// Generated macro for impl_697 (impl)
macro_rules! Depcrate_readimpl_697 {
() => {
// Module: crate::read
// Provides: {"impl_697"}
// Dependencies: {}
impl < 'b , 'c , T > Deref for Reference < 'b , 'c , T > where T : ? Sized + 'static , { type Target = T ; fn deref (& self) -> & Self :: Target { match * self { Reference :: Borrowed (b) => b , Reference :: Copied (c) => c , } } }
};
}
