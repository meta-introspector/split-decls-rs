macro_rules! mev_exclude {
    () => {
        # [proc_macro] # [decl2 (fn , name = "mev_exclude" , vis = "pub" , hash = "7cb0b085")] pub fn mev_exclude (input : TokenStream) -> TokenStream { mev_protection :: mev_exclude_impl (input) }
    };
}

mev_exclude!();