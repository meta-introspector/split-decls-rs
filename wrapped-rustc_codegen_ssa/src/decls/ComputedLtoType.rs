macro_rules! ComputedLtoType {
    () => {
        # [doc = " Actual LTO type we end up choosing based on multiple factors."] pub (crate) enum ComputedLtoType { No , Thin , Fat , }
    };
}

ComputedLtoType!()