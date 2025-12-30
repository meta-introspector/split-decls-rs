// Generated macro for impl_2724 (impl)
macro_rules! Depcrate_fnsimpl_2724 {
() => {
// Module: crate::fns
// Provides: {"impl_2724"}
// Dependencies: {}
impl < F , T , E > Fn1 < Result < T , E > > for MapOkFn < F > where F : Fn1 < T > , { fn call (& self , arg : Result < T , E >) -> Self :: Output { arg . map (| x | self . 0 . call (x)) } }
};
}
