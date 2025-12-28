macro_rules! deps {
    () => {
        NestedName!();
        IsCtorDtorConversion!();
        SubstitutionTable!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl IsCtorDtorConversion for NestedName { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { self . prefix () . map (| p | p . is_ctor_dtor_conversion (subs)) . unwrap_or (false) } }
    };
}

impl_100!();