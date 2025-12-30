// Generated macro for impl_175 (impl)
macro_rules! Depcrate_bnimpl_175 {
() => {
// Module: crate::bn
// Provides: {"impl_175"}
// Dependencies: {}
impl Ord for BigNumRef { fn cmp (& self , oth : & BigNumRef) -> Ordering { unsafe { ffi :: BN_cmp (self . as_ptr () , oth . as_ptr ()) . cmp (& 0) } } }
};
}
