macro_rules! quote_each_token {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! quote_each_token { ($ tokens : ident $ ($ tts : tt) *) => { $ crate :: quote_tokens_with_context ! ($ tokens (@ @ @ @ @ @ $ ($ tts) *) (@ @ @ @ @ $ ($ tts) * @) (@ @ @ @ $ ($ tts) * @ @) (@ @ @ $ (($ tts)) * @ @ @) (@ @ $ ($ tts) * @ @ @ @) (@ $ ($ tts) * @ @ @ @ @) ($ ($ tts) * @ @ @ @ @ @)) ; } ; }
    };
}

quote_each_token!()