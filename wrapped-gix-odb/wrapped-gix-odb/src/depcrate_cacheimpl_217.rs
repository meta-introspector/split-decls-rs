// Generated macro for impl_217 (impl)
macro_rules! Depcrate_cacheimpl_217 {
() => {
// Module: crate::cache
// Provides: {"impl_217"}
// Dependencies: {}
impl Cache < crate :: store :: Handle < Arc < crate :: Store > > > { # [doc = " No op, as we are containing an arc handle already."] pub fn into_arc (self) -> std :: io :: Result < Cache < crate :: store :: Handle < Arc < crate :: Store > > > > { Ok (self) } }
};
}
