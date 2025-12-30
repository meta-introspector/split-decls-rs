// Generated macro for impl_420 (impl)
macro_rules! Depcrate___macros_sync_unsafe_cellimpl_420 {
() => {
// Module: crate::__macros::sync_unsafe_cell
// Provides: {"impl_420"}
// Dependencies: {}
impl < T > SyncUnsafeCell < T > { # [inline] pub const fn new (value : T) -> Self { Self { value : UnsafeCell :: new (value) , } } # [inline] pub fn into_inner (self) -> T { self . value . into_inner () } }
};
}
