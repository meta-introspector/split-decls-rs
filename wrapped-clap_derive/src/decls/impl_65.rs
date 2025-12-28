macro_rules! deps {
    () => {
        Method!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl ToTokens for Method { fn to_tokens (& self , ts : & mut TokenStream) { let Method { name , args } = self ; let tokens = quote ! (.# name (# args)) ; tokens . to_tokens (ts) ; } }
    };
}

impl_65!();