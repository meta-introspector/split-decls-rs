macro_rules! deps {
    () => {
        Intern!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl ToTokens for Intern { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let sig = & self . signature ; let ty = self . pat_and_tys . to_vec () ; let interned_pat = ty . first () . expect ("at least one pat; this is a bug") ; let interned_pat = & interned_pat . pat ; let wrapper_struct = self . interned_struct_path . to_token_stream () ; let method = quote ! { # sig { # wrapper_struct :: new (self , # interned_pat) } } ; method . to_tokens (tokens) ; } }
    };
}

impl_14!()