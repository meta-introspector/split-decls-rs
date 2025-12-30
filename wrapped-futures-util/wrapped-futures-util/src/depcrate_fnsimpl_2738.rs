// Generated macro for impl_2738 (impl)
macro_rules! Depcrate_fnsimpl_2738 {
() => {
// Module: crate::fns
// Provides: {"impl_2738"}
// Dependencies: {}
impl < 'a , F , T , E > FnMut1 < & 'a Result < T , E > > for InspectErrFn < F > where F : FnMut1 < & 'a E , Output = () > , { fn call_mut (& mut self , arg : & 'a Result < T , E >) -> Self :: Output { if let Err (x) = arg { self . 0 . call_mut (x) } } }
};
}
