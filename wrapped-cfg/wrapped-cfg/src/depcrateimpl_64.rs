// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl IntoIterator for CfgOptions { type Item = < FxHashSet < CfgAtom > as IntoIterator > :: Item ; type IntoIter = < FxHashSet < CfgAtom > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { < FxHashSet < CfgAtom > as IntoIterator > :: into_iter (self . enabled) } }
};
}
