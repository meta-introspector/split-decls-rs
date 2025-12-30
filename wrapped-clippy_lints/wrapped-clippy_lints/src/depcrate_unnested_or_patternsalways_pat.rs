// Generated macro for always_pat (macro)
macro_rules! Depcrate_unnested_or_patternsalways_pat {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"always_pat"}
// Dependencies: {}
# [doc = " Match `$scrutinee` against `$pat` and extract `$then` from it."] # [doc = " Panics if there is no match."] macro_rules ! always_pat { ($ scrutinee : expr , $ pat : pat => $ then : expr) => { match $ scrutinee { $ pat => $ then , _ => unreachable ! () , } } ; }
};
}
