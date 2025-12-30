// Generated macro for SpecFromIterNested (trait)
macro_rules! Depcrate_vec_spec_from_iter_nestedSpecFromIterNested {
() => {
// Module: crate::vec::spec_from_iter_nested
// Provides: {"SpecFromIterNested"}
// Dependencies: {}
# [doc = " Another specialization trait for Vec::from_iter"] # [doc = " necessary to manually prioritize overlapping specializations"] # [doc = " see [`SpecFromIter`](super::SpecFromIter) for details."] pub (super) trait SpecFromIterNested < T , I > { fn from_iter (iter : I) -> Self ; }
};
}
