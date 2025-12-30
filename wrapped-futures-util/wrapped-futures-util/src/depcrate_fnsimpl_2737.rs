// Generated macro for impl_2737 (impl)
macro_rules! Depcrate_fnsimpl_2737 {
() => {
// Module: crate::fns
// Provides: {"impl_2737"}
// Dependencies: {}
impl < 'a , F , T , E > FnOnce1 < & 'a Result < T , E > > for InspectErrFn < F > where F : FnOnce1 < & 'a E , Output = () > , { type Output = () ; fn call_once (self , arg : & 'a Result < T , E >) -> Self :: Output { if let Err (x) = arg { self . 0 . call_once (x) } } }
};
}
