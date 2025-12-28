macro_rules! as_if_expr {
    () => {
        fn as_if_expr (element : & SyntaxElement) -> Option < ast :: IfExpr > { let mut node = element . as_node () ? . clone () ; if let Some (stmt) = ast :: ExprStmt :: cast (node . clone ()) { node = stmt . expr () ? . syntax () . clone () ; } ast :: IfExpr :: cast (node) }
    };
}

as_if_expr!();