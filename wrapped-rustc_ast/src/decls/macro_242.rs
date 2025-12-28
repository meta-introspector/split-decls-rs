macro_rules! deps {
    () => {
        Ty!();
        WherePredicate!();
        AssocItem!();
        Stmt!();
        FieldDef!();
        Pat!();
        Item!();
        PatField!();
        Crate!();
        ForeignItem!();
        Variant!();
        Arm!();
        GenericParam!();
        ExprField!();
        Param!();
        Expr!();
    };
}

macro_rules! macro_242 {
    () => {
        deps!();
        impl_has_node_id ! (Arm , AssocItem , Crate , Expr , ExprField , FieldDef , ForeignItem , GenericParam , Item , Param , Pat , PatField , Stmt , Ty , Variant , WherePredicate ,) ;
    };
}

macro_242!()