macro_rules! custom_lt {
    () => {
        fn custom_lt (s : & str) -> Lifetime { Lifetime :: new (s , Span :: call_site ()) }
    };
}

custom_lt!()