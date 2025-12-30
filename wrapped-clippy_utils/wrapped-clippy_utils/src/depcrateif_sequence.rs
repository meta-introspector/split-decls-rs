// Generated macro for if_sequence (function)
macro_rules! Depcrateif_sequence {
() => {
// Module: crate
// Provides: {"if_sequence"}
// Dependencies: {}
# [doc = " Returns the list of condition expressions and the list of blocks in a"] # [doc = " sequence of `if/else`."] # [doc = " E.g., this returns `([a, b], [c, d, e])` for the expression"] # [doc = " `if a { c } else if b { d } else { e }`."] pub fn if_sequence < 'tcx > (mut expr : & 'tcx Expr < 'tcx >) -> (Vec < & 'tcx Expr < 'tcx > > , Vec < & 'tcx Block < 'tcx > >) { let mut conds = Vec :: new () ; let mut blocks : Vec < & Block < '_ > > = Vec :: new () ; while let Some (higher :: IfOrIfLet { cond , then , r#else }) = higher :: IfOrIfLet :: hir (expr) { conds . push (cond) ; if let ExprKind :: Block (block , _) = then . kind { blocks . push (block) ; } else { panic ! ("ExprKind::If node is not an ExprKind::Block") ; } if let Some (else_expr) = r#else { expr = else_expr ; } else { break ; } } if ! blocks . is_empty () && let ExprKind :: Block (block , _) = expr . kind { blocks . push (block) ; } (conds , blocks) }
};
}
