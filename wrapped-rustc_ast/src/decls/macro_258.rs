macro_rules! deps {
    () => {
        PatField!();
        Param!();
        Arm!();
        GenericParam!();
        Crate!();
        FieldDef!();
        WherePredicate!();
        ExprField!();
        Expr!();
        Variant!();
    };
}

macro_rules! macro_258 {
    () => {
        deps!();
        impl_has_attrs ! (const SUPPORTS_CUSTOM_INNER_ATTRS : bool = false , Arm , Crate , Expr , ExprField , FieldDef , GenericParam , Param , PatField , Variant , WherePredicate ,) ;
    };
}

macro_258!();