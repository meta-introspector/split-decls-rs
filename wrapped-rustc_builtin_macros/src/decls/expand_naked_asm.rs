macro_rules! expand_naked_asm {
    () => {
        pub (super) fn expand_naked_asm < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { ExpandResult :: Ready (match parse_args (ecx , sp , tts , AsmMacro :: NakedAsm) { Ok (args) => { let ExpandResult :: Ready (mac) = expand_preparsed_asm (ecx , AsmMacro :: NakedAsm , args) else { return ExpandResult :: Retry (()) ; } ; let expr = match mac { Ok (inline_asm) => Box :: new (ast :: Expr { id : ast :: DUMMY_NODE_ID , kind : ast :: ExprKind :: InlineAsm (Box :: new (inline_asm)) , span : sp , attrs : ast :: AttrVec :: new () , tokens : None , }) , Err (guar) => DummyResult :: raw_expr (sp , Some (guar)) , } ; MacEager :: expr (expr) } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
    };
}

expand_naked_asm!();