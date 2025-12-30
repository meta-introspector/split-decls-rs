// Generated macro for impl_30 (impl)
macro_rules! Depcrate_cacheimpl_30 {
() => {
// Module: crate::cache
// Provides: {"impl_30"}
// Dependencies: {}
impl < S , R > AsyncCache < S , R > where S : BundleStream + Stream , { pub async fn prefetch (& self) { let pin = unsafe { Pin :: new_unchecked (& self . stream) } ; unsafe { PinMut :: as_mut (& mut pin . borrow_mut ()) . get_unchecked_mut () } . prefetch_async () . await ; } }
};
}
