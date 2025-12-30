// Generated macro for impl_2729 (impl)
macro_rules! Depcrate_fnsimpl_2729 {
() => {
// Module: crate::fns
// Provides: {"impl_2729"}
// Dependencies: {}
impl < F , T , E > Fn1 < Result < T , E > > for MapErrFn < F > where F : Fn1 < E > , { fn call (& self , arg : Result < T , E >) -> Self :: Output { arg . map_err (| x | self . 0 . call (x)) } }
};
}
