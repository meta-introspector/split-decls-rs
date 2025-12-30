// Generated macro for impl_10404 (impl)
macro_rules! Depcrate_uninit_vecimpl_10404 {
() => {
// Module: crate::uninit_vec
// Provides: {"impl_10404"}
// Dependencies: {}
impl TargetVec < '_ > { pub fn has_capacity (self) -> bool { ! matches ! (self . init_kind , Some (VecInitKind :: New | VecInitKind :: Default)) } }
};
}
