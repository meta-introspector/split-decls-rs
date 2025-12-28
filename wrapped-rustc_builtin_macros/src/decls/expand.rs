macro_rules! expand {
    () => {
        pub (crate) fn expand < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let (ty , pat) = match parse_pat_ty (cx , tts) { Ok (parsed) => parsed , Err (err) => { return ExpandResult :: Ready (DummyResult :: any (sp , err . emit ())) ; } } ; ExpandResult :: Ready (base :: MacEager :: ty (cx . ty (sp , ast :: TyKind :: Pat (ty , pat)))) }
    };
}

expand!()