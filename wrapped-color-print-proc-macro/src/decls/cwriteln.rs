macro_rules! cwriteln {
    () => {
        # [doc = " The same as `writeln!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cwriteln (input : TokenStream) -> TokenStream { get_macro ("writeln" , input , true) }
    };
}

cwriteln!();