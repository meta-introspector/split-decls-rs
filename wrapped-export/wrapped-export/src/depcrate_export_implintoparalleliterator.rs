// Generated macro for IntoParallelIterator (trait)
macro_rules! Depcrate_export_implIntoParallelIterator {
() => {
// Module: crate::export_impl
// Provides: {"IntoParallelIterator"}
// Dependencies: {}
# [cfg (not (feature = "rayon"))] trait IntoParallelIterator : IntoIterator + Sized { fn into_par_iter (self) -> < Self as IntoIterator > :: IntoIter { self . into_iter () } }
};
}
