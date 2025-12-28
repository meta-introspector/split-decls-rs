macro_rules! deps {
    () => {
        Item!();
        Ty!();
        AssocItem!();
        ExprField!();
        FieldDef!();
        GenericParam!();
        Param!();
        Stmt!();
        ForeignItem!();
        PatField!();
        Crate!();
        Arm!();
        WherePredicate!();
        Variant!();
        Expr!();
        Pat!();
    };
}

macro_rules! macro_242 {
    () => {
        deps!();
        impl_has_node_id ! (Arm , AssocItem , Crate , Expr , ExprField , FieldDef , ForeignItem , GenericParam , Item , Param , Pat , PatField , Stmt , Ty , Variant , WherePredicate ,) ;
    };
}

macro_242!();