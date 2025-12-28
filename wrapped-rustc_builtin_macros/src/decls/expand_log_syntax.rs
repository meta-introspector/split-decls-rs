macro_rules! expand_log_syntax {
    () => {
        pub (crate) fn expand_log_syntax < 'cx > (_cx : & 'cx mut ExtCtxt < '_ > , sp : rustc_span :: Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { println ! ("{}" , pprust :: tts_to_string (& tts)) ; ExpandResult :: Ready (DummyResult :: any_valid (sp)) }
    };
}

expand_log_syntax!()