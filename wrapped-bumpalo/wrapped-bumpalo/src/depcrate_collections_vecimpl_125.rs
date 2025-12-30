// Generated macro for impl_125 (impl)
macro_rules! Depcrate_collections_vecimpl_125 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (feature = "boxed")] impl < 'bump , T : 'bump > From < Vec < 'bump , T > > for crate :: boxed :: Box < 'bump , [T] > { fn from (v : Vec < 'bump , T >) -> crate :: boxed :: Box < 'bump , [T] > { v . into_boxed_slice () } }
};
}
