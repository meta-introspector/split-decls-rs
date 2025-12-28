macro_rules! deps {
    () => {
        ExpandRequires!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl AttrProcMacro for ExpandRequires { fn expand < 'cx > (& self , ecx : & 'cx mut ExtCtxt < '_ > , span : Span , annotation : TokenStream , annotated : TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { expand_requires_tts (ecx , span , annotation , annotated) } }
    };
}

impl_315!();