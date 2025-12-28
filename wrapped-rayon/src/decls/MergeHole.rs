macro_rules! MergeHole {
    () => {
        struct MergeHole < T > { start : * mut T , end : * mut T , dest : * mut T , }
    };
}

MergeHole!();