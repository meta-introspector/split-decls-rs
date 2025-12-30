// Generated macro for impl_2723 (impl)
macro_rules! Depcrate_fnsimpl_2723 {
() => {
// Module: crate::fns
// Provides: {"impl_2723"}
// Dependencies: {}
impl < F , T , E > FnMut1 < Result < T , E > > for MapOkFn < F > where F : FnMut1 < T > , { fn call_mut (& mut self , arg : Result < T , E >) -> Self :: Output { arg . map (| x | self . 0 . call_mut (x)) } }
};
}
