macro_rules! expand_format_args_nl {
    () => {
        pub (crate) fn expand_format_args_nl < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { expand_format_args_impl (ecx , sp , tts , true) }
    };
}

expand_format_args_nl!()