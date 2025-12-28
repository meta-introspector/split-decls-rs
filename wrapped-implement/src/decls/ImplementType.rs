macro_rules! ImplementType {
    () => {
        struct ImplementType { type_name : String , generics : Vec < ImplementType > , # [doc = " The best span for diagnostics."] span : proc_macro2 :: Span , }
    };
}

ImplementType!()