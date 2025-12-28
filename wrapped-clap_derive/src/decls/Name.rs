macro_rules! Name {
    () => {
        # [derive (Clone)] pub (crate) enum Name { Derived (Ident) , Assigned (TokenStream) , }
    };
}

Name!()