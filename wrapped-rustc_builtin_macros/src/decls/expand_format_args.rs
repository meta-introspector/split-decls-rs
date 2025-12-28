macro_rules! expand_format_args {
    () => {
        pub (crate) fn expand_format_args < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { expand_format_args_impl (ecx , sp , tts , false) }
    };
}

expand_format_args!();