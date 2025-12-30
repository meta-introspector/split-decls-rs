// Generated macro for impl_185 (impl)
macro_rules! Depcrate_utils_wakers_vec_waker_vecimpl_185 {
() => {
// Module: crate::utils::wakers::vec::waker_vec
// Provides: {"impl_185"}
// Dependencies: {}
impl WakerVec { # [doc = " Create a new instance of `WakerVec`."] pub (crate) fn new (len : usize) -> Self { let readiness = Arc :: new (Mutex :: new (ReadinessVec :: new (len))) ; let wakers = (0 .. len) . map (| i | Arc :: new (InlineWakerVec :: new (i , readiness . clone ())) . into ()) . collect () ; Self { wakers , readiness } } pub (crate) fn get (& self , index : usize) -> Option < & Waker > { self . wakers . get (index) } # [doc = " Access the `Readiness`."] pub (crate) fn readiness (& self) -> MutexGuard < '_ , ReadinessVec > { self . readiness . lock () . unwrap () } # [doc = " Resize the `WakerVec` to the new size."] pub (crate) fn resize (& mut self , len : usize) { let mut index = self . wakers . len () ; self . wakers . resize_with (len , | | { let ret = Arc :: new (InlineWakerVec :: new (index , self . readiness . clone ())) . into () ; index += 1 ; ret }) ; let mut readiness = self . readiness . lock () . unwrap () ; readiness . resize (len) ; } }
};
}
