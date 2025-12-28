macro_rules! invert_boolean_expression_legacy {
    () => {
        pub (crate) fn invert_boolean_expression_legacy (expr : ast :: Expr) -> ast :: Expr { invert_special_case_legacy (& expr) . unwrap_or_else (| | make :: expr_prefix (T ! [!] , expr) . into ()) }
    };
}

invert_boolean_expression_legacy!()