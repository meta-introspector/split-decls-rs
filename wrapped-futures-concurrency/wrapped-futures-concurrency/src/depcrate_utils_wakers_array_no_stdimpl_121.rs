// Generated macro for impl_121 (impl)
macro_rules! Depcrate_utils_wakers_array_no_stdimpl_121 {
() => {
// Module: crate::utils::wakers::array::no_std
// Provides: {"impl_121"}
// Dependencies: {}
impl < const N : usize > WakerArray < N > { # [doc = " Create a new instance of `WakerArray`."] pub (crate) fn new () -> Self { let readiness = ReadinessArray :: new () ; Self { readiness } } pub (crate) fn get (& self , _index : usize) -> Option < & Waker > { self . readiness . parent_waker () } # [doc = " Access the `Readiness`."] pub (crate) fn readiness (& mut self) -> ReadinessArrayRef < '_ , N > { ReadinessArrayRef { inner : & mut self . readiness , } } }
};
}
