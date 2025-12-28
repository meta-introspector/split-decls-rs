macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! MockItemContent {
    () => {
        deps!();
        enum MockItemContent { Fn (Box < MockFunction >) , Tokens (TokenStream) }
    };
}

MockItemContent!()