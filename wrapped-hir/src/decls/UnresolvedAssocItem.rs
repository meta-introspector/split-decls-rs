macro_rules! UnresolvedAssocItem {
    () => {
        # [derive (Debug)] pub struct UnresolvedAssocItem { pub expr_or_pat : InFile < ExprOrPatPtr > , }
    };
}

UnresolvedAssocItem!()