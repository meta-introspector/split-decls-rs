// Generated macro for impl_110 (impl)
macro_rules! Depcrate_addressimpl_110 {
() => {
// Module: crate::address
// Provides: {"impl_110"}
// Dependencies: {}
impl < T > SendError < T > { pub fn into_inner (self) -> T { match self { SendError :: Full (msg) | SendError :: Closed (msg) => msg , } } }
};
}
