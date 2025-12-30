// Generated macro for check (function)
macro_rules! Depcrate_casts_borrow_as_ptrcheck {
() => {
// Module: crate::casts::borrow_as_ptr
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , cast_expr : & 'tcx Expr < '_ > , cast_to : & 'tcx Ty < '_ > , msrv : Msrv ,) -> bool { if let TyKind :: Ptr (target) = cast_to . kind && ! matches ! (target . ty . kind , TyKind :: TraitObject (..)) && let ExprKind :: AddrOf (BorrowKind :: Ref , mutability , e) = cast_expr . kind && ! is_lint_allowed (cx , BORROW_AS_PTR , expr . hir_id) && ! is_expr_temporary_value (cx , e) && ! is_from_proc_macro (cx , expr) { let mut app = Applicability :: MachineApplicable ; let snip = snippet_with_context (cx , e . span , cast_expr . span . ctxt () , ".." , & mut app) . 0 ; let (suggestion , span) = if msrv . meets (cx , msrvs :: RAW_REF_OP) { let span = if has_enclosing_paren (snippet_with_applicability (cx , expr . span , "" , & mut app)) { expr . span . with_lo (expr . span . lo () + BytePos (1)) . with_hi (expr . span . hi () - BytePos (1)) } else { expr . span } ; (format ! ("&raw {} {snip}" , mutability . ptr_str ()) , span) } else { let Some (std_or_core) = std_or_core (cx) else { return false ; } ; let macro_name = match mutability { Mutability :: Not => "addr_of" , Mutability :: Mut => "addr_of_mut" , } ; (format ! ("{std_or_core}::ptr::{macro_name}!({snip})") , expr . span) } ; span_lint_and_sugg (cx , BORROW_AS_PTR , span , "borrow as raw pointer" , "try" , suggestion , app) ; return true ; } false }
};
}
