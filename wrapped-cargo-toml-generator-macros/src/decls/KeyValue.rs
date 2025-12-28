macro_rules! deps {
    () => {
        InlineTable!();
    };
}

macro_rules! KeyValue {
    () => {
        deps!();
        # [derive (Debug)] pub enum KeyValue { Simple (Ident , LitStr) , Block (Ident , proc_macro2 :: TokenStream) , List (Ident , proc_macro2 :: TokenStream) , InlineTable (Ident , proc_macro2 :: TokenStream) , }
    };
}

KeyValue!()