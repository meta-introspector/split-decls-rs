macro_rules! deps {
    () => {
        MockItemTraitImpl!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl MockItemTraitImpl { # [doc = " Are all of this traits's methods static?"] fn all_static (& self) -> bool { self . methods . all_static () } fn phantom_default_inits (& self) -> Vec < TokenStream > { phantom_default_inits (& self . generics) } fn phantom_fields (& self) -> Vec < TokenStream > { phantom_fields (& self . generics) } }
    };
}

impl_72!();