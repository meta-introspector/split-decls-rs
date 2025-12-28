macro_rules! deps {
    () => {
        InMemoryDir!();
    };
}

macro_rules! impl_from_tuple_for_inmemorydir {
    () => {
        deps!();
        # [doc = " Create an `impl _ for InMemoryDir` for a generic tuple"] # [doc = ""] # [doc = " Must pass in names for each tuple parameter for"] # [doc = " - internal variable name"] # [doc = " - `Path` type"] # [doc = " - `Data` type"] macro_rules ! impl_from_tuple_for_inmemorydir { ($ ($ var : ident $ path : ident $ data : ident) ,+) => { impl <$ ($ path : Into < PathBuf >, $ data : IntoData) ,+> From < ($ (($ path , $ data)) ,+ ,) > for InMemoryDir { fn from (files : ($ (($ path , $ data)) ,+,)) -> Self { let ($ ($ var) ,+ ,) = files ; let files = [$ (($ var . 0 . into () , $ var . 1 . into_data ())) ,+] ; files . into () } } } ; }
    };
}

impl_from_tuple_for_inmemorydir!()