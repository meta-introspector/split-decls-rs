// Generated macro for impl_2750 (impl)
macro_rules! Depcrate_fnsimpl_2750 {
() => {
// Module: crate::fns
// Provides: {"impl_2750"}
// Dependencies: {}
impl < A , T > FnOnce1 < A > for IntoFn < T > where A : Into < T > , { type Output = T ; fn call_once (self , arg : A) -> Self :: Output { arg . into () } }
};
}
