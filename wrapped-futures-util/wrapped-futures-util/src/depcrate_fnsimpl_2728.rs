// Generated macro for impl_2728 (impl)
macro_rules! Depcrate_fnsimpl_2728 {
() => {
// Module: crate::fns
// Provides: {"impl_2728"}
// Dependencies: {}
impl < F , T , E > FnMut1 < Result < T , E > > for MapErrFn < F > where F : FnMut1 < E > , { fn call_mut (& mut self , arg : Result < T , E >) -> Self :: Output { arg . map_err (| x | self . 0 . call_mut (x)) } }
};
}
