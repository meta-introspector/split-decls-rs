macro_rules! Method {
    () => {
        # [derive (Clone)] pub (crate) struct Method { name : Ident , args : TokenStream , }
    };
}

Method!();