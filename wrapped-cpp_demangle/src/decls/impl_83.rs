macro_rules! deps {
    () => {
        SubstitutionTable!();
        IsCtorDtorConversion!();
        Name!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl IsCtorDtorConversion for Name { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { match * self { Name :: Unscoped (ref unscoped) => unscoped . is_ctor_dtor_conversion (subs) , Name :: Nested (ref nested) => nested . is_ctor_dtor_conversion (subs) , Name :: Local (_) | Name :: UnscopedTemplate (..) => false , } } }
    };
}

impl_83!();