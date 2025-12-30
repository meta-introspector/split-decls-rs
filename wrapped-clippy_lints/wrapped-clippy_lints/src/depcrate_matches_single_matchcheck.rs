// Generated macro for check (function)
macro_rules! Depcrate_matches_single_matchcheck {
() => {
// Module: crate::matches::single_match
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check < 'tcx > (cx : & LateContext < 'tcx > , ex : & 'tcx Expr < '_ > , arms : & 'tcx [Arm < '_ >] , expr : & 'tcx Expr < '_ > , contains_comments : bool ,) { if let [arm1 , arm2] = arms && ! arms . iter () . any (| arm | arm . guard . is_some () || arm . pat . span . from_expansion ()) && ! expr . span . from_expansion () && ! matches ! (arm1 . pat . kind , PatKind :: Or (..)) { let els = if is_unit_expr (peel_blocks (arm2 . body)) && ! empty_arm_has_comment (cx , arm2 . body . span) { None } else if let ExprKind :: Block (block , _) = arm2 . body . kind { if matches ! ((block . stmts , block . expr) , ([] , Some (_)) | ([_] , None)) { return ; } Some (arm2 . body) } else { return ; } ; let typeck = cx . typeck_results () ; if * typeck . expr_ty (ex) . peel_refs () . kind () != ty :: Bool || is_lint_allowed (cx , MATCH_BOOL , ex . hir_id) { let mut v = PatVisitor { typeck , has_enum : false , } ; if v . visit_pat (arm2 . pat) . is_break () { return ; } if v . has_enum { let cx = PatCtxt { tcx : cx . tcx , typeck , arena : DroplessArena :: default () , } ; let mut state = PatState :: Other ; if ! (state . add_pat (& cx , arm2 . pat) || state . add_pat (& cx , arm1 . pat)) { return ; } } report_single_pattern (cx , ex , arm1 , expr , els , contains_comments) ; } } }
};
}
