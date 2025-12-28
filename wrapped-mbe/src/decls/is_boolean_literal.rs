macro_rules! is_boolean_literal {
    () => {
        fn is_boolean_literal (lit : & tt :: Literal < Span >) -> bool { matches ! (lit . symbol . as_str () , "true" | "false") }
    };
}

is_boolean_literal!();