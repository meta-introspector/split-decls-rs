macro_rules! make_ule {
    () => {
        # [doc = " Full docs for this proc macro can be found on the [`zerovec`](https://docs.rs/zerovec) crate."] # [proc_macro_attribute] pub fn make_ule (attr : TokenStream , item : TokenStream) -> TokenStream { let input = parse_macro_input ! (item as DeriveInput) ; let attr = parse_macro_input ! (attr as Ident) ; TokenStream :: from (make_ule :: make_ule_impl (attr , input)) }
    };
}

make_ule!();