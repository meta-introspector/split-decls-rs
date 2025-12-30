// Generated macro for impl_106 (impl)
macro_rules! Depcrate_fs_feature_gitimpl_106 {
() => {
// Module: crate::fs::feature::git
// Provides: {"impl_106"}
// Dependencies: {}
impl GitCache { pub fn has_anything_for (& self , index : & Path) -> bool { self . repos . iter () . any (| e | e . has_path (index)) } pub fn get (& self , index : & Path , prefix_lookup : bool) -> f :: Git { self . repos . iter () . find (| repo | repo . has_path (index)) . map (| repo | repo . search (index , prefix_lookup)) . unwrap_or_default () } }
};
}
