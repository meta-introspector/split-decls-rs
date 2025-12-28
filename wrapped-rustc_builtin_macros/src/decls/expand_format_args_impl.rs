macro_rules! expand_format_args_impl {
    () => {
        fn expand_format_args_impl < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , mut sp : Span , tts : TokenStream , nl : bool ,) -> MacroExpanderResult < 'cx > { sp = ecx . with_def_site_ctxt (sp) ; ExpandResult :: Ready (match parse_args (ecx , sp , tts) { Ok (input) => { let ExpandResult :: Ready (mac) = make_format_args (ecx , input , nl) else { return ExpandResult :: Retry (()) ; } ; match mac { Ok (format_args) => { MacEager :: expr (ecx . expr (sp , ExprKind :: FormatArgs (Box :: new (format_args)))) } Err (guar) => MacEager :: expr (DummyResult :: raw_expr (sp , Some (guar))) , } } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
    };
}

expand_format_args_impl!()