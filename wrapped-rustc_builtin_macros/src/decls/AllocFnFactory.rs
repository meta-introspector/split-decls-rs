macro_rules! AllocFnFactory {
    () => {
        struct AllocFnFactory < 'a , 'b > { span : Span , ty_span : Span , global : Ident , cx : & 'a ExtCtxt < 'b > , }
    };
}

AllocFnFactory!();