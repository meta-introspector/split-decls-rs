// Generated macro for impl_2739 (impl)
macro_rules! Depcrate_fnsimpl_2739 {
() => {
// Module: crate::fns
// Provides: {"impl_2739"}
// Dependencies: {}
impl < 'a , F , T , E > Fn1 < & 'a Result < T , E > > for InspectErrFn < F > where F : Fn1 < & 'a E , Output = () > , { fn call (& self , arg : & 'a Result < T , E >) -> Self :: Output { if let Err (x) = arg { self . 0 . call (x) } } }
};
}
