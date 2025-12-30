// Generated macro for impl_758 (impl)
macro_rules! Depcrate_readimpl_758 {
() => {
// Module: crate::read
// Provides: {"impl_758"}
// Dependencies: {}
impl < 'b , 'c , T > Deref for Reference < 'b , 'c , T > where T : ? Sized + 'static , { type Target = T ; fn deref (& self) -> & Self :: Target { match * self { Reference :: Borrowed (b) => b , Reference :: Copied (c) => c , } } }
};
}
