macro_rules! deps {
    () => {
        AssocItem!();
    };
}

macro_rules! PrivateAssocItem {
    () => {
        deps!();
        # [derive (Debug)] pub struct PrivateAssocItem { pub expr_or_pat : InFile < ExprOrPatPtr > , pub item : AssocItem , }
    };
}

PrivateAssocItem!()