macro_rules! cover_let_chain {
    () => {
        pub (crate) fn cover_let_chain (mut expr : ast :: Expr , range : TextRange) -> Option < ast :: Expr > { if ! expr . syntax () . text_range () . contains_range (range) { return None ; } loop { let (chain_expr , rest) = if let ast :: Expr :: BinExpr (bin_expr) = & expr && bin_expr . op_kind () == Some (ast :: BinaryOp :: LogicOp (ast :: LogicOp :: And)) { (bin_expr . rhs () , bin_expr . lhs ()) } else { (Some (expr) , None) } ; if let Some (chain_expr) = chain_expr && chain_expr . syntax () . text_range () . contains_range (range) { break Some (chain_expr) ; } expr = rest ? ; } }
    };
}

cover_let_chain!();