// Generated macro for impl_208 (impl)
macro_rules! Depcrate_blob_unified_diff_implsimpl_208 {
() => {
// Module: crate::blob::unified_diff::impls
// Provides: {"impl_208"}
// Dependencies: {}
impl DiffLineKind { # [doc = " Returns a one-character representation for use in unified diffs."] pub const fn to_prefix (self) -> char { match self { DiffLineKind :: Context => ' ' , DiffLineKind :: Add => '+' , DiffLineKind :: Remove => '-' , } } }
};
}
