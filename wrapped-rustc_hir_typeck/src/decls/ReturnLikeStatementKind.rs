macro_rules! ReturnLikeStatementKind {
    () => {
        pub (crate) enum ReturnLikeStatementKind { Return , Become , }
    };
}

ReturnLikeStatementKind!()