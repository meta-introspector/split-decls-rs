macro_rules! deps {
    () => {
        GenericParam!();
        WherePredicate!();
        ExprField!();
        PatField!();
        Variant!();
        Arm!();
        FieldDef!();
        Param!();
    };
}

macro_rules! macro_248 {
    () => {
        deps!();
        impl_has_tokens_none ! (Arm , ExprField , FieldDef , GenericParam , Param , PatField , Variant , WherePredicate) ;
    };
}

macro_248!();