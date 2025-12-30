// Generated macro for SpanlessEq (struct)
macro_rules! Depcrate_hir_utilsSpanlessEq {
() => {
// Module: crate::hir_utils
// Provides: {"SpanlessEq"}
// Dependencies: {}
# [doc = " Type used to check whether two ast are the same. This is different from the"] # [doc = " operator `==` on ast types as this operator would compare true equality with"] # [doc = " ID and span."] # [doc = ""] # [doc = " Note that some expressions kinds are not considered but could be added."] pub struct SpanlessEq < 'a , 'tcx > { # [doc = " Context used to evaluate constant expressions."] cx : & 'a LateContext < 'tcx > , maybe_typeck_results : Option < (& 'tcx TypeckResults < 'tcx > , & 'tcx TypeckResults < 'tcx >) > , allow_side_effects : bool , expr_fallback : Option < Box < SpanlessEqCallback < 'a > > > , path_check : PathCheck , }
};
}
