// Generated macro for impl_284 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_284 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_284"}
// Dependencies: {}
impl < S > TryStream < S > { fn into_inner (self) -> S { self . stream . inner . into_inner () . inner } fn as_ref (& self) -> & S { & self . stream . inner . get_ref () . inner } fn as_mut (& mut self) -> & mut S { & mut self . stream . inner . get_mut () . inner } }
};
}
