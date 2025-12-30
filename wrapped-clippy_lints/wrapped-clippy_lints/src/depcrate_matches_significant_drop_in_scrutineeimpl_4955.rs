// Generated macro for impl_4955 (impl)
macro_rules! Depcrate_matches_significant_drop_in_scrutineeimpl_4955 {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"impl_4955"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for SigDropHelper < '_ , 'tcx > { fn visit_expr (& mut self , ex : & 'tcx Expr < '_ >) { if self . sig_drop_holder == SigDropHolder :: Moved { return ; } let sig_drop_holder_before = core :: mem :: take (& mut self . sig_drop_holder) ; let sig_drop_spans_before = core :: mem :: take (& mut self . sig_drop_spans) ; let parent_expr_before = self . parent_expr . replace (ex) ; match ex . kind { ExprKind :: Block (..) | ExprKind :: Match (_ , _ , MatchSource :: AwaitDesugar) => () , _ => walk_expr (self , ex) , } if let Some (parent_ex) = parent_expr_before { match parent_ex . kind { ExprKind :: Assign (lhs , _ , _) | ExprKind :: AssignOp (_ , lhs , _) if lhs . hir_id == ex . hir_id && self . sig_drop_holder == SigDropHolder :: Moved => { self . replace_current_sig_drop (parent_ex . span , true , 0) ; } , _ => { self . try_move_sig_drop (ex , parent_ex) ; } , } } self . sig_drop_holder = std :: cmp :: max (self . sig_drop_holder , sig_drop_holder_before) ; if self . sig_drop_holder != SigDropHolder :: Moved { let mut sig_drop_spans = sig_drop_spans_before ; sig_drop_spans . append (& mut self . sig_drop_spans) ; self . sig_drop_spans = sig_drop_spans ; } self . parent_expr = parent_expr_before ; } }
};
}
