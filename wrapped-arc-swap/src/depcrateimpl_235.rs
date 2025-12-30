// Generated macro for impl_235 (impl)
macro_rules! Depcrateimpl_235 {
() => {
// Module: crate
// Provides: {"impl_235"}
// Dependencies: {}
impl < T : RefCnt , S : Strategy < T > > Drop for ArcSwapAny < T , S > { fn drop (& mut self) { let ptr = * self . ptr . get_mut () ; unsafe { self . strategy . wait_for_readers (ptr , & self . ptr) ; T :: dec (ptr) ; } } }
};
}
