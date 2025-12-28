macro_rules! expand_global_asm {
    () => {
        pub (super) fn expand_global_asm < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { ExpandResult :: Ready (match parse_args (ecx , sp , tts , AsmMacro :: GlobalAsm) { Ok (args) => { let ExpandResult :: Ready (mac) = expand_preparsed_asm (ecx , AsmMacro :: GlobalAsm , args) else { return ExpandResult :: Retry (()) ; } ; match mac { Ok (inline_asm) => MacEager :: items (smallvec ! [Box :: new (ast :: Item { attrs : ast :: AttrVec :: new () , id : ast :: DUMMY_NODE_ID , kind : ast :: ItemKind :: GlobalAsm (Box :: new (inline_asm)) , vis : ast :: Visibility { span : sp . shrink_to_lo () , kind : ast :: VisibilityKind :: Inherited , tokens : None , } , span : sp , tokens : None , })]) , Err (guar) => DummyResult :: any (sp , guar) , } } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
    };
}

expand_global_asm!()