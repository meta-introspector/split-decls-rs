// Generated macro for impl_2745 (impl)
macro_rules! Depcrate_fnsimpl_2745 {
() => {
// Module: crate::fns
// Provides: {"impl_2745"}
// Dependencies: {}
impl < F , T , E > FnMut1 < Result < T , E > > for UnwrapOrElseFn < F > where F : FnMut1 < E , Output = T > , { fn call_mut (& mut self , arg : Result < T , E >) -> Self :: Output { arg . unwrap_or_else (| x | self . 0 . call_mut (x)) } }
};
}
