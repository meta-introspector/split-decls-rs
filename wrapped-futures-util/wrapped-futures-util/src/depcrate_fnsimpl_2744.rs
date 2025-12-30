// Generated macro for impl_2744 (impl)
macro_rules! Depcrate_fnsimpl_2744 {
() => {
// Module: crate::fns
// Provides: {"impl_2744"}
// Dependencies: {}
impl < F , T , E > FnOnce1 < Result < T , E > > for UnwrapOrElseFn < F > where F : FnOnce1 < E , Output = T > , { type Output = T ; fn call_once (self , arg : Result < T , E >) -> Self :: Output { arg . unwrap_or_else (| x | self . 0 . call_once (x)) } }
};
}
