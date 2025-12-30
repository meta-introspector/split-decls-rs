// Generated macro for impl_2717 (impl)
macro_rules! Depcrate_fnsimpl_2717 {
() => {
// Module: crate::fns
// Provides: {"impl_2717"}
// Dependencies: {}
impl < F , A > FnOnce1 < A > for InspectFn < F > where F : for < 'a > FnOnce1 < & 'a A , Output = () > , { type Output = A ; fn call_once (self , arg : A) -> Self :: Output { self . 0 . call_once (& arg) ; arg } }
};
}
