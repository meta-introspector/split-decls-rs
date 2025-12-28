macro_rules! DocCommentToken {
    () => {
        pub (crate) struct DocCommentToken { doc_token : SyntaxToken , prefix_len : TextSize , }
    };
}

DocCommentToken!();