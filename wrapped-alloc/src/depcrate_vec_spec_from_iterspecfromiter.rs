// Generated macro for SpecFromIter (trait)
macro_rules! Depcrate_vec_spec_from_iterSpecFromIter {
() => {
// Module: crate::vec::spec_from_iter
// Provides: {"SpecFromIter"}
// Dependencies: {}
# [doc = " Specialization trait used for Vec::from_iter"] # [doc = ""] # [doc = " ## The delegation graph:"] # [doc = ""] # [doc = " ```text"] # [doc = " +-------------+"] # [doc = " |FromIterator |"] # [doc = " +-+-----------+"] # [doc = "   |"] # [doc = "   v"] # [doc = " +-+---------------------------------+  +---------------------+"] # [doc = " |SpecFromIter                    +---->+SpecFromIterNested   |"] # [doc = " |where I:                        |  |  |where I:             |"] # [doc = " |  Iterator (default)------------+  |  |  Iterator (default) |"] # [doc = " |  vec::IntoIter                 |  |  |  TrustedLen         |"] # [doc = " |  InPlaceCollect--(fallback to)-+  |  +---------------------+"] # [doc = " +-----------------------------------+"] # [doc = " ```"] pub (super) trait SpecFromIter < T , I > { fn from_iter (iter : I) -> Self ; }
};
}
