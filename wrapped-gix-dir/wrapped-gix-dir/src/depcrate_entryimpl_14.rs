// Generated macro for impl_14 (impl)
macro_rules! Depcrate_entryimpl_14 {
() => {
// Module: crate::entry
// Provides: {"impl_14"}
// Dependencies: {}
impl From < gix_pathspec :: search :: Match < '_ > > for PathspecMatch { fn from (m : gix_pathspec :: search :: Match < '_ >) -> Self { if m . is_excluded () { PathspecMatch :: Excluded } else { m . kind . into () } } }
};
}
