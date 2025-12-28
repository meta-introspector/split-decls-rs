macro_rules! deps {
    () => {
        Item!();
        AssocItemKind!();
    };
}

macro_rules! AssocItem {
    () => {
        deps!();
        # [doc = " Represents associated items."] # [doc = " These include items in `impl` and `trait` definitions."] pub type AssocItem = Item < AssocItemKind > ;
    };
}

AssocItem!()