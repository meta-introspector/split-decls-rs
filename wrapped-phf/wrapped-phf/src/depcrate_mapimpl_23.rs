// Generated macro for impl_23 (impl)
macro_rules! Depcrate_mapimpl_23 {
() => {
// Module: crate::map
// Provides: {"impl_23"}
// Dependencies: {}
impl < K , V > PartialEq for Map < K , V > where K : PartialEq , V : PartialEq , { fn eq (& self , other : & Self) -> bool { self . key == other . key && self . disps == other . disps && self . entries == other . entries } }
};
}
