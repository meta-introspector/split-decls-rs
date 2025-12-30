// Generated macro for SpanlessHash (struct)
macro_rules! Depcrate_hir_utilsSpanlessHash {
() => {
// Module: crate::hir_utils
// Provides: {"SpanlessHash"}
// Dependencies: {}
# [doc = " Type used to hash an ast element. This is different from the `Hash` trait"] # [doc = " on ast types as this"] # [doc = " trait would consider IDs and spans."] # [doc = ""] # [doc = " All expressions kind are hashed, but some might have a weaker hash."] pub struct SpanlessHash < 'a , 'tcx > { # [doc = " Context used to evaluate constant expressions."] cx : & 'a LateContext < 'tcx > , maybe_typeck_results : Option < & 'tcx TypeckResults < 'tcx > > , s : FxHasher , path_check : PathCheck , }
};
}
