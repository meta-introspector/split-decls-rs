// Generated macro for impl_2734 (impl)
macro_rules! Depcrate_fnsimpl_2734 {
() => {
// Module: crate::fns
// Provides: {"impl_2734"}
// Dependencies: {}
impl < 'a , F , T , E > Fn1 < & 'a Result < T , E > > for InspectOkFn < F > where F : Fn1 < & 'a T , Output = () > , { fn call (& self , arg : & 'a Result < T , E >) -> Self :: Output { if let Ok (x) = arg { self . 0 . call (x) } } }
};
}
