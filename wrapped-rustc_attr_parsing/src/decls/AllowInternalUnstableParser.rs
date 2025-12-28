macro_rules! AllowInternalUnstableParser {
    () => {
        pub (crate) struct AllowInternalUnstableParser ;
    };
}

AllowInternalUnstableParser!();