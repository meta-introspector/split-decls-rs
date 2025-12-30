// Generated macro for impl_232 (impl)
macro_rules! Depcrate_memoryimpl_232 {
() => {
// Module: crate::memory
// Provides: {"impl_232"}
// Dependencies: {}
impl Proxy < Cache < crate :: store :: Handle < Arc < crate :: Store > > > > { # [doc = " No op, as we are containing an arc handle already."] pub fn into_arc (self) -> std :: io :: Result < Proxy < Cache < crate :: store :: Handle < Arc < crate :: Store > > > > > { Ok (self) } }
};
}
