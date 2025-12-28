macro_rules! JsonUnexpected {
    () => {
        struct JsonUnexpected < 'a > (de :: Unexpected < 'a >) ;
    };
}

JsonUnexpected!()