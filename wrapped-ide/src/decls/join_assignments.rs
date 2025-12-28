macro_rules! join_assignments {
    () => {
        fn join_assignments (edit : & mut TextEditBuilder , prev : & SyntaxElement , next : & SyntaxElement ,) -> Option < () > { let let_stmt = ast :: LetStmt :: cast (prev . as_node () ? . clone ()) ? ; if let_stmt . eq_token () . is_some () { cov_mark :: hit ! (join_assignments_already_initialized) ; return None ; } let let_ident_pat = match let_stmt . pat () ? { ast :: Pat :: IdentPat (it) => it , _ => return None , } ; let expr_stmt = ast :: ExprStmt :: cast (next . as_node () ? . clone ()) ? ; let bin_expr = match expr_stmt . expr () ? { ast :: Expr :: BinExpr (it) => it , _ => return None , } ; if ! matches ! (bin_expr . op_kind () ?, ast :: BinaryOp :: Assignment { op : None }) { return None ; } let lhs = bin_expr . lhs () ? ; let name_ref = expr_as_name_ref (& lhs) ? ; if name_ref . to_string () != let_ident_pat . syntax () . to_string () { cov_mark :: hit ! (join_assignments_mismatch) ; return None ; } edit . delete (let_stmt . semicolon_token () ? . text_range () . cover (lhs . syntax () . text_range ())) ; Some (()) }
    };
}

join_assignments!()