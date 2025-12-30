// Generated macro for check (function)
macro_rules! Depcrate_methods_type_id_on_boxcheck {
() => {
// Module: crate::methods::type_id_on_box
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , receiver : & Expr < '_ > , call_span : Span) { let recv_adjusts = cx . typeck_results () . expr_adjustments (receiver) ; if let Some (Adjustment { target : recv_ty , .. }) = recv_adjusts . last () && let ty :: Ref (_ , ty , _) = recv_ty . kind () && let ty :: Adt (adt , args) = ty . kind () && adt . is_box () && let inner_box_ty = args . type_at (0) && let ty :: Dynamic (..) = inner_box_ty . kind () { let ty_name = with_forced_trimmed_paths ! (ty . to_string ()) ; span_lint_and_then (cx , TYPE_ID_ON_BOX , call_span , format ! ("calling `.type_id()` on `{ty_name}`") , | diag | { let derefs = recv_adjusts . iter () . filter (| adj | matches ! (adj . kind , Adjust :: Deref (None))) . count () ; diag . note ("this returns the type id of the literal type `Box<_>` instead of the \
                    type id of the boxed value, which is most likely not what you want" ,) . note (format ! ("if this is intentional, use `TypeId::of::<{ty_name}>()` instead, \
                    which makes it more clear")) ; if is_subtrait_of_any (cx , inner_box_ty) { let mut sugg = "*" . repeat (derefs + 1) ; sugg += & snippet (cx , receiver . span , "<expr>") ; diag . span_suggestion (receiver . span , "consider dereferencing first" , format ! ("({sugg})") , Applicability :: MaybeIncorrect ,) ; } } ,) ; } }
};
}
