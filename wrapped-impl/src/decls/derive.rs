macro_rules! derive {
    () => {
        pub fn derive (input : & DeriveInput) -> TokenStream { match try_expand (input) { Ok (expanded) => expanded , Err (error) => fallback :: expand (input , error) , } }
    };
}

derive!();