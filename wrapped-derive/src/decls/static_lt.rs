macro_rules! static_lt {
    () => {
        fn static_lt () -> Lifetime { Lifetime :: new ("'static" , Span :: call_site ()) }
    };
}

static_lt!()