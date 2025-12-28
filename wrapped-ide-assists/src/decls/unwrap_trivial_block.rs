macro_rules! unwrap_trivial_block {
    () => {
        pub (crate) fn unwrap_trivial_block (block_expr : ast :: BlockExpr) -> ast :: Expr { extract_trivial_expression (& block_expr) . filter (| expr | ! expr . syntax () . text () . contains_char ('\n')) . unwrap_or_else (| | block_expr . into ()) }
    };
}

unwrap_trivial_block!()