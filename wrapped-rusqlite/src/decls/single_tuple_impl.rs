macro_rules! deps {
    () => {
        Params!();
        Statement!();
        Result!();
        ToSql!();
    };
}

macro_rules! single_tuple_impl {
    () => {
        deps!();
        macro_rules ! single_tuple_impl { ($ count : literal : $ (($ field : tt $ ftype : ident)) ,* $ (,) ?) => { impl <$ ($ ftype ,) *> Sealed for ($ ($ ftype ,) *) where $ ($ ftype : ToSql ,) * { } impl <$ ($ ftype ,) *> Params for ($ ($ ftype ,) *) where $ ($ ftype : ToSql ,) * { fn __bind_in (self , stmt : & mut Statement <'_ >) -> Result < () > { stmt . ensure_parameter_count ($ count) ?; $ ({ debug_assert ! ($ field < $ count) ; stmt . raw_bind_parameter ($ field + 1 , self .$ field) ?; }) + Ok (()) } } } }
    };
}

single_tuple_impl!()