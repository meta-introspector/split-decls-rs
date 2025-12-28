macro_rules! ceprint {
    () => {
        # [doc = " The same as `eprint!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn ceprint (input : TokenStream) -> TokenStream { get_macro ("eprint" , input , false) }
    };
}

ceprint!()