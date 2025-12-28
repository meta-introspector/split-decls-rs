macro_rules! find_fn_or_blocks {
    () => {
        pub (crate) fn find_fn_or_blocks (sema : & Semantics < '_ , RootDatabase > , token : & SyntaxToken ,) -> Vec < SyntaxNode > { let find_ancestors = | token : SyntaxToken | { let token_kind = token . kind () ; for anc in sema . token_ancestors_with_macros (token) { let node = match_ast ! { match anc { ast :: Fn (fn_) => fn_ . syntax () . clone () , ast :: ClosureExpr (c) => c . syntax () . clone () , ast :: BlockExpr (blk) => { match blk . modifier () { Some (ast :: BlockModifier :: Async (_)) => blk . syntax () . clone () , Some (ast :: BlockModifier :: Try (_)) if token_kind != T ! [return] => blk . syntax () . clone () , _ => continue , } } , _ => continue , } } ; return Some (node) ; } None } ; sema . descend_into_macros (token . clone ()) . into_iter () . filter_map (find_ancestors) . collect_vec () }
    };
}

find_fn_or_blocks!()