// Generated macro for check (function)
macro_rules! Depcrate_ptr_mut_from_refcheck {
() => {
// Module: crate::ptr::mut_from_ref
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , sig : & FnSig < '_ > , body : Option < & Body < 'tcx > >) { let FnRetTy :: Return (ty) = sig . decl . output else { return } ; for (out , mutability , out_span) in get_lifetimes (ty) { if mutability != Some (Mutability :: Mut) { continue ; } let out_region = cx . tcx . named_bound_var (out . hir_id) ; let args_immut_refs : Option < Vec < Span > > = sig . decl . inputs . iter () . flat_map (get_lifetimes) . filter (| & (lt , _ , _) | cx . tcx . named_bound_var (lt . hir_id) == out_region) . map (| (_ , mutability , span) | (mutability == Some (Mutability :: Not)) . then_some (span)) . collect () ; if let Some (args_immut_refs) = args_immut_refs && ! args_immut_refs . is_empty () && body . is_none_or (| body | sig . header . is_unsafe () || contains_unsafe_block (cx , body . value)) { span_lint_and_then (cx , MUT_FROM_REF , out_span , "mutable borrow from immutable input(s)" , | diag | { let ms = MultiSpan :: from_spans (args_immut_refs) ; diag . span_note (ms , "immutable borrow here") ; } ,) ; } } }
};
}
