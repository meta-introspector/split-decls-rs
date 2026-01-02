mkuse!{use rustc_ast :: tokenstream :: TokenStream ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_expand :: base :: { DummyResult , ExpandResult , ExtCtxt , MacroExpanderResult } ;}

macro_rules! expand_log_syntax_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_log_syntax in module {}", module_path!());
    };
}

mkfn!{
    expand_log_syntax_introspect!();
    pub (crate) fn expand_log_syntax < 'cx > (_cx : & 'cx mut ExtCtxt < '_ > , sp : rustc_span :: Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { println ! ("{}" , pprust :: tts_to_string (& tts)) ; ExpandResult :: Ready (DummyResult :: any_valid (sp)) }
}