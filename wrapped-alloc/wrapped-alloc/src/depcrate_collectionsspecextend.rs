// Generated macro for SpecExtend (trait)
macro_rules! Depcrate_collectionsSpecExtend {
() => {
// Module: crate::collections
// Provides: {"SpecExtend"}
// Dependencies: {}
# [doc = " An intermediate trait for specialization of `Extend`."] # [doc (hidden)] # [cfg (not (no_global_oom_handling))] trait SpecExtend < I : IntoIterator > { # [doc = " Extends `self` with the contents of the given iterator."] fn spec_extend (& mut self , iter : I) ; }
};
}
