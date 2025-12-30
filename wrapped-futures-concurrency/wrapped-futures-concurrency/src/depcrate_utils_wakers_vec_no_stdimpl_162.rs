// Generated macro for impl_162 (impl)
macro_rules! Depcrate_utils_wakers_vec_no_stdimpl_162 {
() => {
// Module: crate::utils::wakers::vec::no_std
// Provides: {"impl_162"}
// Dependencies: {}
impl WakerVec { # [doc = " Create a new instance of `WakerArray`."] pub (crate) fn new (_len : usize) -> Self { let readiness = ReadinessVec :: new () ; Self { readiness } } pub (crate) fn get (& self , _index : usize) -> Option < & Waker > { self . readiness . parent_waker () } # [doc = " Access the `Readiness`."] pub (crate) fn readiness (& mut self) -> ReadinessVecRef < '_ > { ReadinessVecRef { inner : & mut self . readiness , } } # [doc = " Resize the `WakerVec` to the new size."] pub (crate) fn resize (& mut self , len : usize) { self . readiness . resize (len) ; } }
};
}
