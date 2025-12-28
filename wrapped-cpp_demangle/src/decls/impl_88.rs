macro_rules! deps {
    () => {
        SubstitutionTable!();
        IsCtorDtorConversion!();
        UnscopedName!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl IsCtorDtorConversion for UnscopedName { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { match * self { UnscopedName :: Unqualified (ref name) | UnscopedName :: Std (ref name) => { name . is_ctor_dtor_conversion (subs) } } } }
    };
}

impl_88!();