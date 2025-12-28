macro_rules! cwrite {
    () => {
        # [doc = " The same as `write!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cwrite (input : TokenStream) -> TokenStream { get_macro ("write" , input , true) }
    };
}

cwrite!()