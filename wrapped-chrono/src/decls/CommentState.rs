macro_rules! CommentState {
    () => {
        enum CommentState { Start , Next (usize) , Escape (usize) , }
    };
}

CommentState!();