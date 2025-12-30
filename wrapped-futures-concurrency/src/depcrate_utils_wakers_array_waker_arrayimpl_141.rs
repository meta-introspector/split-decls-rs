// Generated macro for impl_141 (impl)
macro_rules! Depcrate_utils_wakers_array_waker_arrayimpl_141 {
() => {
// Module: crate::utils::wakers::array::waker_array
// Provides: {"impl_141"}
// Dependencies: {}
impl < const N : usize > WakerArray < N > { # [doc = " Create a new instance of `WakerArray`."] pub (crate) fn new () -> Self { let readiness = Arc :: new (Mutex :: new (ReadinessArray :: new ())) ; Self { wakers : array :: from_fn (| i | { Arc :: new (InlineWakerArray :: new (i , readiness . clone ())) . into () }) , readiness , } } pub (crate) fn get (& self , index : usize) -> Option < & Waker > { self . wakers . get (index) } # [doc = " Access the `Readiness`."] pub (crate) fn readiness (& mut self) -> MutexGuard < '_ , ReadinessArray < N > > { self . readiness . as_ref () . lock () . unwrap () } }
};
}
