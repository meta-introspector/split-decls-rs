macro_rules! deps {
    () => {
        NestedName!();
        SubstitutionTable!();
        IsCtorDtorConversion!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl IsCtorDtorConversion for NestedName { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { self . prefix () . map (| p | p . is_ctor_dtor_conversion (subs)) . unwrap_or (false) } }
    };
}

impl_100!()