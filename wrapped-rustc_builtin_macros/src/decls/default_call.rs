macro_rules! default_call {
    () => {
        fn default_call (cx : & ExtCtxt < '_ > , span : Span) -> Box < ast :: Expr > { let default_ident = cx . std_path (& [kw :: Default , sym :: Default , kw :: Default]) ; cx . expr_call_global (span , default_ident , ThinVec :: new ()) }
    };
}

default_call!();