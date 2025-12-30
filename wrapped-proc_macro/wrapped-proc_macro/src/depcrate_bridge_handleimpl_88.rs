// Generated macro for impl_88 (impl)
macro_rules! Depcrate_bridge_handleimpl_88 {
() => {
// Module: crate::bridge::handle
// Provides: {"impl_88"}
// Dependencies: {}
impl < T > OwnedStore < T > { pub (super) fn new (counter : & 'static AtomicU32) -> Self { assert_ne ! (counter . load (Ordering :: Relaxed) , 0) ; OwnedStore { counter , data : BTreeMap :: new () } } }
};
}
