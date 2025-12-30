// Generated macro for coalesce (function)
macro_rules! Depcrate_adaptors_coalescecoalesce {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"coalesce"}
// Dependencies: {}
# [doc = " Create a new `Coalesce`."] pub fn coalesce < I , F > (iter : I , f : F) -> Coalesce < I , F > where I : Iterator , { Coalesce { last : None , iter , f , } }
};
}
