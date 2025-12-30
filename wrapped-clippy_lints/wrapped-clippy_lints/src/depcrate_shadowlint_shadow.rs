// Generated macro for lint_shadow (function)
macro_rules! Depcrate_shadowlint_shadow {
() => {
// Module: crate::shadow
// Provides: {"lint_shadow"}
// Dependencies: {}
fn lint_shadow (cx : & LateContext < '_ > , pat : & Pat < '_ > , shadowed : HirId , span : Span) { let (lint , msg) = match find_init (cx , pat . hir_id) { Some ((expr , _)) if is_self_shadow (cx , pat , expr , shadowed) => { let msg = format ! ("`{}` is shadowed by itself in `{}`" , snippet (cx , pat . span , "_") , snippet (cx , expr . span , "..")) ; (SHADOW_SAME , msg) } , Some ((expr , except)) if is_local_used_except (cx , expr , shadowed , except) => { let msg = format ! ("`{}` is shadowed" , snippet (cx , pat . span , "_")) ; (SHADOW_REUSE , msg) } , _ => { let msg = format ! ("`{}` shadows a previous, unrelated binding" , snippet (cx , pat . span , "_")) ; (SHADOW_UNRELATED , msg) } , } ; span_lint_and_then (cx , lint , span , msg , | diag | { diag . span_note (cx . tcx . hir_span (shadowed) , "previous binding is here") ; }) ; }
};
}
