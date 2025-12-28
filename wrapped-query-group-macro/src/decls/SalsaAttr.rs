macro_rules! SalsaAttr {
    () => {
        struct SalsaAttr { name : String , tts : TokenStream , span : Span , }
    };
}

SalsaAttr!();