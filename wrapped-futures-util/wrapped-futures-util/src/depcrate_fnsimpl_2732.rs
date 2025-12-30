// Generated macro for impl_2732 (impl)
macro_rules! Depcrate_fnsimpl_2732 {
() => {
// Module: crate::fns
// Provides: {"impl_2732"}
// Dependencies: {}
impl < 'a , F , T , E > FnOnce1 < & 'a Result < T , E > > for InspectOkFn < F > where F : FnOnce1 < & 'a T , Output = () > , { type Output = () ; fn call_once (self , arg : & 'a Result < T , E >) -> Self :: Output { if let Ok (x) = arg { self . 0 . call_once (x) } } }
};
}
