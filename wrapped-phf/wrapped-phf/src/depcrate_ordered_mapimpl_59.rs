// Generated macro for impl_59 (impl)
macro_rules! Depcrate_ordered_mapimpl_59 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_59"}
// Dependencies: {}
impl < K , V > PartialEq for OrderedMap < K , V > where K : PartialEq , V : PartialEq , { fn eq (& self , other : & Self) -> bool { self . key == other . key && self . disps == other . disps && self . idxs == other . idxs && self . entries == other . entries } }
};
}
