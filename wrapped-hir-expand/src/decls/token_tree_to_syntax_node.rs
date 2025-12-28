macro_rules! deps {
    () => {
        ExpansionSpanMap!();
        ExpandDatabase!();
        ExpandTo!();
    };
}

macro_rules! token_tree_to_syntax_node {
    () => {
        deps!();
        pub (crate) fn token_tree_to_syntax_node (db : & dyn ExpandDatabase , tt : & tt :: TopSubtree , expand_to : ExpandTo , edition : parser :: Edition ,) -> (Parse < SyntaxNode > , ExpansionSpanMap) { let entry_point = match expand_to { ExpandTo :: Statements => syntax_bridge :: TopEntryPoint :: MacroStmts , ExpandTo :: Items => syntax_bridge :: TopEntryPoint :: MacroItems , ExpandTo :: Pattern => syntax_bridge :: TopEntryPoint :: Pattern , ExpandTo :: Type => syntax_bridge :: TopEntryPoint :: Type , ExpandTo :: Expr => syntax_bridge :: TopEntryPoint :: Expr , } ; syntax_bridge :: token_tree_to_syntax_node (tt , entry_point , & mut | ctx | ctx . edition (db) , edition) }
    };
}

token_tree_to_syntax_node!();