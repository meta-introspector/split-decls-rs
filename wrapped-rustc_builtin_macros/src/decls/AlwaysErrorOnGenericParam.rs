macro_rules! AlwaysErrorOnGenericParam {
    () => {
        struct AlwaysErrorOnGenericParam < 'a , 'b > { cx : & 'a ExtCtxt < 'b > , }
    };
}

AlwaysErrorOnGenericParam!();