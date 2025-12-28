macro_rules! Match {
    () => {
        enum Match { # [doc = " There was no match."] None , # [doc = " No additional data is provided as part of the match."] Normal , # [doc = " The range of text to copy from the originating item name"] GlobRange (Range < usize >) , }
    };
}

Match!();