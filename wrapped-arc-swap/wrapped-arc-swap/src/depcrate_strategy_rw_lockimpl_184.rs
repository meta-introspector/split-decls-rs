// Generated macro for impl_184 (impl)
macro_rules! Depcrate_strategy_rw_lockimpl_184 {
() => {
// Module: crate::strategy::rw_lock
// Provides: {"impl_184"}
// Dependencies: {}
impl < T : RefCnt > InnerStrategy < T > for RwLock < () > { type Protected = T ; unsafe fn load (& self , storage : & AtomicPtr < T :: Base >) -> T { let _guard = self . read () . expect ("We don't panic in here") ; let ptr = storage . load (Ordering :: Acquire) ; let ptr = T :: from_ptr (ptr as * const T :: Base) ; T :: inc (& ptr) ; ptr } unsafe fn wait_for_readers (& self , _ : * const T :: Base , _ : & AtomicPtr < T :: Base >) { drop (self . write () . expect ("We don't panic in here")) ; } }
};
}
