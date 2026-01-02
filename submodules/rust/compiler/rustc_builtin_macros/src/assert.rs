mkmod!{context, { 
                getname!(context);
                getsrc!(context);
                getpath!(context);
                get_deps!(context);
                get_crates!(context);
                mkinclude!(context);
                 
            }}
mkuse!{use rustc_ast :: token :: Delimiter ;}
mkuse!{use rustc_ast :: tokenstream :: { DelimSpan , TokenStream } ;}
mkuse!{use rustc_ast :: { DelimArgs , Expr , ExprKind , MacCall , Path , PathSegment , UnOp , token } ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_errors :: PResult ;}
mkuse!{use rustc_expand :: base :: { DummyResult , ExpandResult , ExtCtxt , MacEager , MacroExpanderResult } ;}
mkuse!{use rustc_parse :: exp ;}
mkuse!{use rustc_parse :: parser :: Parser ;}
mkuse!{use rustc_span :: { DUMMY_SP , Ident , Span , Symbol , sym } ;}
mkuse!{use thin_vec :: thin_vec ;}
mkuse!{use crate :: edition_panic :: use_panic_2021 ;}
mkuse!{use crate :: errors ;}

macro_rules! expand_assert_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_assert in module {}", module_path!());
    };
}

mkfn!{
    expand_assert_introspect!();
    pub (crate) fn expand_assert < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , span : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let Assert { cond_expr , custom_message } = match parse_assert (cx , span , tts) { Ok (assert) => assert , Err (err) => { let guar = err . emit () ; return ExpandResult :: Ready (DummyResult :: any (span , guar)) ; } } ; let call_site_span = cx . with_call_site_ctxt (span) ; let panic_path = | | { if use_panic_2021 (span) { Path { span : call_site_span , segments : cx . std_path (& [sym :: panic , sym :: panic_2021]) . into_iter () . map (| ident | PathSegment :: from_ident (ident)) . collect () , tokens : None , } } else { Path :: from_ident (Ident :: new (sym :: panic , call_site_span)) } } ; let expr = if let Some (tokens) = custom_message { let then = cx . expr (call_site_span , ExprKind :: MacCall (Box :: new (MacCall { path : panic_path () , args : Box :: new (DelimArgs { dspan : DelimSpan :: from_single (call_site_span) , delim : Delimiter :: Parenthesis , tokens , }) , })) ,) ; expr_if_not (cx , call_site_span , cond_expr , then , None) } else if cx . ecfg . features . generic_assert () { context :: Context :: new (cx , call_site_span) . build (cond_expr , panic_path ()) } else { let then = cx . expr_call_global (call_site_span , cx . std_path (& [sym :: panicking , sym :: panic]) , thin_vec ! [cx . expr_str (DUMMY_SP , Symbol :: intern (& format ! ("assertion failed: {}" , pprust :: expr_to_string (& cond_expr))) ,)] ,) ; expr_if_not (cx , call_site_span , cond_expr , then , None) } ; ExpandResult :: Ready (MacEager :: expr (expr)) }
}
mkitem!{mkstruct!{struct Assert { cond_expr : Box < Expr > , custom_message : Option < TokenStream > , }}}

macro_rules! expr_if_not_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_if_not in module {}", module_path!());
    };
}

mkfn!{
    expr_if_not_introspect!();
    fn expr_if_not (cx : & ExtCtxt < '_ > , span : Span , cond : Box < Expr > , then : Box < Expr > , els : Option < Box < Expr > > ,) -> Box < Expr > { cx . expr_if (span , cx . expr (span , ExprKind :: Unary (UnOp :: Not , cond)) , then , els) }
}

macro_rules! parse_assert_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_assert in module {}", module_path!());
    };
}

mkfn!{
    parse_assert_introspect!();
    fn parse_assert < 'a > (cx : & ExtCtxt < 'a > , sp : Span , stream : TokenStream) -> PResult < 'a , Assert > { let mut parser = cx . new_parser_from_tts (stream) ; if parser . token == token :: Eof { return Err (cx . dcx () . create_err (errors :: AssertRequiresBoolean { span : sp })) ; } let cond_expr = parser . parse_expr () ? ; if parser . token == token :: Semi { cx . dcx () . emit_err (errors :: AssertRequiresExpression { span : sp , token : parser . token . span }) ; parser . bump () ; } let custom_message = if let token :: Literal (token :: Lit { kind : token :: Str , .. }) = parser . token . kind { let comma = parser . prev_token . span . shrink_to_hi () ; cx . dcx () . emit_err (errors :: AssertMissingComma { span : parser . token . span , comma }) ; parse_custom_message (& mut parser) } else if parser . eat (exp ! (Comma)) { parse_custom_message (& mut parser) } else { None } ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok (Assert { cond_expr , custom_message }) }
}

macro_rules! parse_custom_message_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_custom_message in module {}", module_path!());
    };
}

mkfn!{
    parse_custom_message_introspect!();
    fn parse_custom_message (parser : & mut Parser < '_ >) -> Option < TokenStream > { let ts = parser . parse_tokens () ; if ! ts . is_empty () { Some (ts) } else { None } }
}