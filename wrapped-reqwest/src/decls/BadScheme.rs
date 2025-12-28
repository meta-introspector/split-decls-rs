macro_rules! BadScheme {
    () => {
        # [derive (Debug)] pub (crate) struct BadScheme ;
    };
}

BadScheme!();