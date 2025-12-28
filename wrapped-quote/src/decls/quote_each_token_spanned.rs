macro_rules! quote_each_token_spanned {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! quote_each_token_spanned { ($ tokens : ident $ span : ident $ ($ tts : tt) *) => { $ crate :: quote_tokens_with_context_spanned ! { $ tokens $ span (@ @ @ @ @ @ $ ($ tts) *) (@ @ @ @ @ $ ($ tts) * @) (@ @ @ @ $ ($ tts) * @ @) (@ @ @ $ (($ tts)) * @ @ @) (@ @ $ ($ tts) * @ @ @ @) (@ $ ($ tts) * @ @ @ @ @) ($ ($ tts) * @ @ @ @ @ @) } } ; }
    };
}

quote_each_token_spanned!();