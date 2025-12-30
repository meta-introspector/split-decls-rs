// Generated macro for impl_13 (impl)
macro_rules! Depcrate_entryimpl_13 {
() => {
// Module: crate::entry
// Provides: {"impl_13"}
// Dependencies: {}
impl From < gix_pathspec :: search :: MatchKind > for PathspecMatch { fn from (kind : gix_pathspec :: search :: MatchKind) -> Self { match kind { gix_pathspec :: search :: MatchKind :: Always => Self :: Always , gix_pathspec :: search :: MatchKind :: Prefix => Self :: Prefix , gix_pathspec :: search :: MatchKind :: WildcardMatch => Self :: WildcardMatch , gix_pathspec :: search :: MatchKind :: Verbatim => Self :: Verbatim , } } }
};
}
