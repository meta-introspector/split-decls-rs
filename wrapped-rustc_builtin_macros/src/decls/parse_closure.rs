macro_rules! parse_closure {
    () => {
        fn parse_closure < 'a > (cx : & mut ExtCtxt < 'a > , span : Span , stream : TokenStream ,) -> PResult < 'a , Box < Expr > > { let mut closure_parser = cx . new_parser_from_tts (stream) ; let coroutine_kind = Some (CoroutineKind :: Gen { span , closure_id : DUMMY_NODE_ID , return_impl_trait_id : DUMMY_NODE_ID , }) ; let mut closure = closure_parser . parse_expr () ? ; match & mut closure . kind { ast :: ExprKind :: Closure (c) => { if let Some (kind) = c . coroutine_kind { cx . dcx () . span_err (kind . span () , "only plain closures allowed in `iter!`") ; } c . coroutine_kind = coroutine_kind ; if closure_parser . token != token :: Eof { closure_parser . unexpected () ? ; } Ok (closure) } _ => { cx . dcx () . span_err (closure . span , "`iter!` body must be a closure") ; Err (closure_parser . unexpected () . unwrap_err ()) } } }
    };
}

parse_closure!()