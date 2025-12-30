// Generated macro for impl_2722 (impl)
macro_rules! Depcrate_fnsimpl_2722 {
() => {
// Module: crate::fns
// Provides: {"impl_2722"}
// Dependencies: {}
impl < F , T , E > FnOnce1 < Result < T , E > > for MapOkFn < F > where F : FnOnce1 < T > , { type Output = Result < F :: Output , E > ; fn call_once (self , arg : Result < T , E >) -> Self :: Output { arg . map (| x | self . 0 . call_once (x)) } }
};
}
