macro_rules! deps {
    () => {
        GenericParam!();
        PatField!();
        Variant!();
        FieldDef!();
        Param!();
        WherePredicate!();
        Crate!();
        Expr!();
        ExprField!();
        Arm!();
    };
}

macro_rules! macro_258 {
    () => {
        deps!();
        impl_has_attrs ! (const SUPPORTS_CUSTOM_INNER_ATTRS : bool = false , Arm , Crate , Expr , ExprField , FieldDef , GenericParam , Param , PatField , Variant , WherePredicate ,) ;
    };
}

macro_258!()