macro_rules! AllowInternalUnsafeParser {
    () => {
        pub (crate) struct AllowInternalUnsafeParser ;
    };
}

AllowInternalUnsafeParser!()