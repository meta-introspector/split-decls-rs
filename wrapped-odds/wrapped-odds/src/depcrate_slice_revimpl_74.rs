// Generated macro for impl_74 (impl)
macro_rules! Depcrate_slice_revimpl_74 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_74"}
// Dependencies: {}
impl < T , U > PartialEq < RevSlice < U > > for RevSlice < T > where T : PartialEq < U > , { fn eq (& self , rhs : & RevSlice < U >) -> bool { self . 0 == rhs . 0 } }
};
}
