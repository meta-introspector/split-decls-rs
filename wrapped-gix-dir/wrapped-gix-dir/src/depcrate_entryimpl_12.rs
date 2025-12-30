// Generated macro for impl_12 (impl)
macro_rules! Depcrate_entryimpl_12 {
() => {
// Module: crate::entry
// Provides: {"impl_12"}
// Dependencies: {}
impl PathspecMatch { pub (crate) fn should_ignore (& self) -> bool { match self { PathspecMatch :: Always | PathspecMatch :: Excluded => true , PathspecMatch :: Prefix | PathspecMatch :: WildcardMatch | PathspecMatch :: Verbatim => false , } } }
};
}
