// Generated macro for impl_65 (impl)
macro_rules! Depcrateimpl_65 {
() => {
// Module: crate
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a CfgOptions { type Item = < & 'a FxHashSet < CfgAtom > as IntoIterator > :: Item ; type IntoIter = < & 'a FxHashSet < CfgAtom > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { < & FxHashSet < CfgAtom > as IntoIterator > :: into_iter (& self . enabled) } }
};
}
