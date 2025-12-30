// Generated macro for impl_2727 (impl)
macro_rules! Depcrate_fnsimpl_2727 {
() => {
// Module: crate::fns
// Provides: {"impl_2727"}
// Dependencies: {}
impl < F , T , E > FnOnce1 < Result < T , E > > for MapErrFn < F > where F : FnOnce1 < E > , { type Output = Result < T , F :: Output > ; fn call_once (self , arg : Result < T , E >) -> Self :: Output { arg . map_err (| x | self . 0 . call_once (x)) } }
};
}
