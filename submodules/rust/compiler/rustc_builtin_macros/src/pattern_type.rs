mkuse!{use rustc_ast :: tokenstream :: TokenStream ;}
mkuse!{use rustc_ast :: { AnonConst , DUMMY_NODE_ID , Ty , TyPat , TyPatKind , ast , token } ;}
mkuse!{use rustc_errors :: PResult ;}
mkuse!{use rustc_expand :: base :: { self , DummyResult , ExpandResult , ExtCtxt , MacroExpanderResult } ;}
mkuse!{use rustc_parse :: exp ;}
mkuse!{use rustc_parse :: parser :: { CommaRecoveryMode , RecoverColon , RecoverComma } ;}
mkuse!{use rustc_span :: Span ;}

macro_rules! expand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand in module {}", module_path!());
    };
}

mkfn!{
    expand_introspect!();
    pub (crate) fn expand < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let (ty , pat) = match parse_pat_ty (cx , tts) { Ok (parsed) => parsed , Err (err) => { return ExpandResult :: Ready (DummyResult :: any (sp , err . emit ())) ; } } ; ExpandResult :: Ready (base :: MacEager :: ty (cx . ty (sp , ast :: TyKind :: Pat (ty , pat)))) }
}

macro_rules! parse_pat_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_pat_ty in module {}", module_path!());
    };
}

mkfn!{
    parse_pat_ty_introspect!();
    fn parse_pat_ty < 'a > (cx : & mut ExtCtxt < 'a > , stream : TokenStream ,) -> PResult < 'a , (Box < Ty > , Box < TyPat >) > { let mut parser = cx . new_parser_from_tts (stream) ; let ty = parser . parse_ty () ? ; parser . expect_keyword (exp ! (Is)) ? ; let pat = pat_to_ty_pat (cx , * parser . parse_pat_no_top_guard (None , RecoverComma :: No , RecoverColon :: No , CommaRecoveryMode :: EitherTupleOrPipe ,) ? ,) ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok ((ty , pat)) }
}

macro_rules! ty_pat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ty_pat in module {}", module_path!());
    };
}

mkfn!{
    ty_pat_introspect!();
    fn ty_pat (kind : TyPatKind , span : Span) -> Box < TyPat > { Box :: new (TyPat { id : DUMMY_NODE_ID , kind , span , tokens : None }) }
}

macro_rules! pat_to_ty_pat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pat_to_ty_pat in module {}", module_path!());
    };
}

mkfn!{
    pat_to_ty_pat_introspect!();
    fn pat_to_ty_pat (cx : & mut ExtCtxt < '_ > , pat : ast :: Pat) -> Box < TyPat > { let kind = match pat . kind { ast :: PatKind :: Range (start , end , include_end) => TyPatKind :: Range (start . map (| value | Box :: new (AnonConst { id : DUMMY_NODE_ID , value })) , end . map (| value | Box :: new (AnonConst { id : DUMMY_NODE_ID , value })) , include_end ,) , ast :: PatKind :: Or (variants) => { TyPatKind :: Or (variants . into_iter () . map (| pat | pat_to_ty_pat (cx , * pat)) . collect ()) } ast :: PatKind :: Err (guar) => TyPatKind :: Err (guar) , _ => TyPatKind :: Err (cx . dcx () . span_err (pat . span , "pattern not supported in pattern types")) , } ; ty_pat (kind , pat . span) }
}