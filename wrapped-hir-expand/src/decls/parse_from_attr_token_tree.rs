macro_rules! parse_from_attr_token_tree {
    () => {
        # [doc = " Parses a `cfg` attribute from the meta"] fn parse_from_attr_token_tree (tt : & TokenTree) -> Option < CfgExpr > { let mut iter = tt . token_trees_and_tokens () . filter (is_not_whitespace) . skip (1) . take_while (is_not_closing_paren) . peekable () ; next_cfg_expr_from_syntax (& mut iter) }
    };
}

parse_from_attr_token_tree!();