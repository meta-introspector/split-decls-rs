// Generated macro for impl_209 (impl)
macro_rules! Depcrate_blob_unified_diff_implsimpl_209 {
() => {
// Module: crate::blob::unified_diff::impls
// Provides: {"impl_209"}
// Dependencies: {}
impl std :: fmt :: Display for HunkHeader { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "@@ -{},{} +{},{} @@" , self . before_hunk_start , self . before_hunk_len , self . after_hunk_start , self . after_hunk_len) } }
};
}
