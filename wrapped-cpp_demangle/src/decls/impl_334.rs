macro_rules! deps {
    () => {
        Substitutable!();
        IsCtorDtorConversion!();
        Prefix!();
        SubstitutionTable!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl ast :: IsCtorDtorConversion for Substitutable { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { match * self { Substitutable :: Prefix (ref prefix) => prefix . is_ctor_dtor_conversion (subs) , _ => false , } } }
    };
}

impl_334!();