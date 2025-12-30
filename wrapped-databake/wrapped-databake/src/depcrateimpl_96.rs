// Generated macro for impl_96 (impl)
macro_rules! Depcrateimpl_96 {
() => {
// Module: crate
// Provides: {"impl_96"}
// Dependencies: {}
impl IntoIterator for CrateEnv { type Item = & 'static str ; type IntoIter = < HashSet < & 'static str > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . 0 . into_inner () . expect ("poison") . into_iter () } }
};
}
