macro_rules! UnresolvedIdent {
    () => {
        # [derive (Debug)] pub struct UnresolvedIdent { pub node : InFile < (ExprOrPatPtr , Option < TextRange >) > , }
    };
}

UnresolvedIdent!();