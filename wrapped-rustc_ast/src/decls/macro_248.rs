macro_rules! deps {
    () => {
        Arm!();
        FieldDef!();
        Variant!();
        WherePredicate!();
        GenericParam!();
        PatField!();
        Param!();
        ExprField!();
    };
}

macro_rules! macro_248 {
    () => {
        deps!();
        impl_has_tokens_none ! (Arm , ExprField , FieldDef , GenericParam , Param , PatField , Variant , WherePredicate) ;
    };
}

macro_248!()