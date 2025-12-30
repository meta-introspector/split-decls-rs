// Generated macro for impl_2733 (impl)
macro_rules! Depcrate_fnsimpl_2733 {
() => {
// Module: crate::fns
// Provides: {"impl_2733"}
// Dependencies: {}
impl < 'a , F , T , E > FnMut1 < & 'a Result < T , E > > for InspectOkFn < F > where F : FnMut1 < & 'a T , Output = () > , { fn call_mut (& mut self , arg : & 'a Result < T , E >) -> Self :: Output { if let Ok (x) = arg { self . 0 . call_mut (x) } } }
};
}
