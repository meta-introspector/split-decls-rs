macro_rules! cformat {
    () => {
        # [doc = " The same as `format!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cformat (input : TokenStream) -> TokenStream { get_macro ("format" , input , false) }
    };
}

cformat!();