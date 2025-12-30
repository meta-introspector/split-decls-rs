// Generated macro for impl_2718 (impl)
macro_rules! Depcrate_fnsimpl_2718 {
() => {
// Module: crate::fns
// Provides: {"impl_2718"}
// Dependencies: {}
impl < F , A > FnMut1 < A > for InspectFn < F > where F : for < 'a > FnMut1 < & 'a A , Output = () > , { fn call_mut (& mut self , arg : A) -> Self :: Output { self . 0 . call_mut (& arg) ; arg } }
};
}
