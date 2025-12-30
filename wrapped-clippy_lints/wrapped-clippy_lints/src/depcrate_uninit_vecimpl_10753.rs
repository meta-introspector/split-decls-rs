// Generated macro for impl_10753 (impl)
macro_rules! Depcrate_uninit_vecimpl_10753 {
() => {
// Module: crate::uninit_vec
// Provides: {"impl_10753"}
// Dependencies: {}
impl TargetVec < '_ > { pub fn has_capacity (self) -> bool { ! matches ! (self . init_kind , Some (VecInitKind :: New | VecInitKind :: Default)) } }
};
}
