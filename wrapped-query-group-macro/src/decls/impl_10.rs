macro_rules! deps {
    () => {
        SetterKind!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl ToTokens for SetterKind { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { match self { SetterKind :: Plain (input_setter) => input_setter . to_tokens (tokens) , SetterKind :: WithDurability (input_setter_with_durability) => { input_setter_with_durability . to_tokens (tokens) } } } }
    };
}

impl_10!()