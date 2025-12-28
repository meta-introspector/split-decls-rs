macro_rules! pick_node_for_resolution {
    () => {
        # [doc = " Returns a suitable node for resolving paths in the current scope. If we create a scope based on"] # [doc = " a statement node, then we can't resolve local variables that were defined in the current scope"] # [doc = " (only in parent scopes). So we find another node, ideally a child of the statement where local"] # [doc = " variable resolution is permitted."] fn pick_node_for_resolution (node : SyntaxNode) -> SyntaxNode { match node . kind () { SyntaxKind :: EXPR_STMT => { if let Some (n) = node . first_child () { cov_mark :: hit ! (cursor_after_semicolon) ; return n ; } } SyntaxKind :: LET_STMT | SyntaxKind :: IDENT_PAT => { if let Some (next) = node . next_sibling () { return pick_node_for_resolution (next) ; } } SyntaxKind :: NAME => { if let Some (parent) = node . parent () { return pick_node_for_resolution (parent) ; } } _ => { } } node }
    };
}

pick_node_for_resolution!();