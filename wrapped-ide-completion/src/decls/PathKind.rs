macro_rules! deps {
    () => {
        AttrCtx!();
        PatternContext!();
        TypeLocation!();
        PathExprCtx!();
        ItemListKind!();
        ExistingDerives!();
    };
}

macro_rules! PathKind {
    () => {
        deps!();
        # [doc = " The kind of path we are completing right now."] # [derive (Debug , PartialEq , Eq)] pub (crate) enum PathKind < 'db > { Expr { expr_ctx : PathExprCtx < 'db > , } , Type { location : TypeLocation , } , Attr { attr_ctx : AttrCtx , } , Derive { existing_derives : ExistingDerives , } , # [doc = " Path in item position, that is inside an (Assoc)ItemList"] Item { kind : ItemListKind , } , Pat { pat_ctx : PatternContext , } , Vis { has_in_token : bool , } , Use , }
    };
}

PathKind!()