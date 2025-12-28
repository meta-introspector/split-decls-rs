macro_rules! invert_boolean_expression {
    () => {
        pub (crate) fn invert_boolean_expression (make : & SyntaxFactory , expr : ast :: Expr) -> ast :: Expr { invert_special_case (make , & expr) . unwrap_or_else (| | make . expr_prefix (T ! [!] , expr) . into ()) }
    };
}

invert_boolean_expression!();