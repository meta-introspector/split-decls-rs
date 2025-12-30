// Generated macro for impl_2746 (impl)
macro_rules! Depcrate_fnsimpl_2746 {
() => {
// Module: crate::fns
// Provides: {"impl_2746"}
// Dependencies: {}
impl < F , T , E > Fn1 < Result < T , E > > for UnwrapOrElseFn < F > where F : Fn1 < E , Output = T > , { fn call (& self , arg : Result < T , E >) -> Self :: Output { arg . unwrap_or_else (| x | self . 0 . call (x)) } }
};
}
