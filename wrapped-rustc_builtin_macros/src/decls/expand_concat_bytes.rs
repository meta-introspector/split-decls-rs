macro_rules! deps {
    () => {
        ConcatBytesMissingLiteral!();
        ConcatBytesBadRepeat!();
    };
}

macro_rules! expand_concat_bytes {
    () => {
        deps!();
        pub (crate) fn expand_concat_bytes (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let ExpandResult :: Ready (mac) = get_exprs_from_tts (cx , tts) else { return ExpandResult :: Retry (()) ; } ; let es = match mac { Ok (es) => es , Err (guar) => return ExpandResult :: Ready (DummyResult :: any (sp , guar)) , } ; let mut accumulator = Vec :: new () ; let mut missing_literals = vec ! [] ; let mut guar = None ; for e in es { match & e . kind { ExprKind :: Array (exprs) => { for expr in exprs { if let Some (elem) = handle_array_element (cx , & mut guar , & mut missing_literals , expr) { accumulator . push (elem) ; } } } ExprKind :: Repeat (expr , count) => { if let ExprKind :: Lit (token_lit) = count . value . kind && let Ok (LitKind :: Int (count_val , _)) = LitKind :: from_token_lit (token_lit) { if let Some (elem) = handle_array_element (cx , & mut guar , & mut missing_literals , expr) { for _ in 0 .. count_val . get () { accumulator . push (elem) ; } } } else { guar = Some (cx . dcx () . emit_err (errors :: ConcatBytesBadRepeat { span : count . value . span }) ,) ; } } & ExprKind :: Lit (token_lit) => match LitKind :: from_token_lit (token_lit) { Ok (LitKind :: Byte (val)) => { accumulator . push (val) ; } Ok (LitKind :: ByteStr (ref byte_sym , _)) => { accumulator . extend_from_slice (byte_sym . as_byte_str ()) ; } _ => { guar . get_or_insert_with (| | invalid_type_err (cx , token_lit , e . span , false)) ; } } , ExprKind :: IncludedBytes (byte_sym) => { accumulator . extend_from_slice (byte_sym . as_byte_str ()) ; } ExprKind :: Err (guarantee) => { guar = Some (* guarantee) ; } ExprKind :: Dummy => cx . dcx () . span_bug (e . span , "concatenating `ExprKind::Dummy`") , _ => { missing_literals . push (e . span) ; } } } ExpandResult :: Ready (if ! missing_literals . is_empty () { let guar = cx . dcx () . emit_err (errors :: ConcatBytesMissingLiteral { spans : missing_literals }) ; MacEager :: expr (DummyResult :: raw_expr (sp , Some (guar))) } else if let Some (guar) = guar { MacEager :: expr (DummyResult :: raw_expr (sp , Some (guar))) } else { let sp = cx . with_def_site_ctxt (sp) ; MacEager :: expr (cx . expr_byte_str (sp , accumulator)) }) }
    };
}

expand_concat_bytes!()