macro_rules! deps {
    () => {
        ExpandEnsures!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl AttrProcMacro for ExpandEnsures { fn expand < 'cx > (& self , ecx : & 'cx mut ExtCtxt < '_ > , span : Span , annotation : TokenStream , annotated : TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { expand_ensures_tts (ecx , span , annotation , annotated) } }
    };
}

impl_316!()