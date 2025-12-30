// Generated macro for impl_260 (impl)
macro_rules! Depcrate_errorsimpl_260 {
() => {
// Module: crate::errors
// Provides: {"impl_260"}
// Dependencies: {}
impl < 'a , G : EmissionGuarantee > Diagnostic < '_ , G > for BreakNonLoop < 'a > { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let mut diag = Diag :: new (dcx , level , fluent :: hir_typeck_break_non_loop) ; diag . span (self . span) ; diag . code (E0571) ; diag . arg ("kind" , self . kind) ; diag . span_label (self . span , fluent :: hir_typeck_label) ; if let Some (head) = self . head { diag . span_label (head , fluent :: hir_typeck_label2) ; } diag . span_suggestion (self . span , fluent :: hir_typeck_suggestion , self . suggestion , Applicability :: MaybeIncorrect ,) ; if let (Some (label) , None) = (self . loop_label , self . break_label) { match self . break_expr_kind { ExprKind :: Path (hir :: QPath :: Resolved (None , hir :: Path { segments : [segment] , res : hir :: def :: Res :: Err , .. } ,)) if label . ident . to_string () == format ! ("'{}" , segment . ident) => { diag . downgrade_to_delayed_bug () ; } _ => { diag . span_suggestion (self . break_expr_span , fluent :: hir_typeck_break_expr_suggestion , label . ident , Applicability :: MaybeIncorrect ,) ; } } } diag } }
};
}
