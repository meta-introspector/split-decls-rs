// Generated macro for impl_79 (impl)
macro_rules! Depcrate_eqimpl_79 {
() => {
// Module: crate::eq
// Provides: {"impl_79"}
// Dependencies: {}
impl < T > ApproxEq for & [T] where T : Copy + ApproxEq , { type Margin = < T as ApproxEq > :: Margin ; fn approx_eq < M : Into < Self :: Margin > > (self , other : Self , margin : M) -> bool { let margin = margin . into () ; if self . len () != other . len () { return false ; } self . iter () . zip (other . iter ()) . all (| (a , b) | a . approx_eq (* b , margin)) } }
};
}
