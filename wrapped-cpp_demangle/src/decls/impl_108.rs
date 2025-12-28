macro_rules! deps {
    () => {
        SubstitutionTable!();
        Prefix!();
        IsCtorDtorConversion!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl IsCtorDtorConversion for Prefix { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool { match * self { Prefix :: Unqualified (ref unqualified) | Prefix :: Nested (_ , ref unqualified) => { unqualified . is_ctor_dtor_conversion (subs) } Prefix :: Template (ref prefix , _) => prefix . is_ctor_dtor_conversion (subs) , _ => false , } } }
    };
}

impl_108!();