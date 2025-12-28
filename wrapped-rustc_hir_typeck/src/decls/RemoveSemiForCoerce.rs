macro_rules! RemoveSemiForCoerce {
    () => {
        pub (crate) struct RemoveSemiForCoerce { pub expr : Span , pub ret : Span , pub semi : Span , }
    };
}

RemoveSemiForCoerce!();