macro_rules! is_not_whitespace {
    () => {
        fn is_not_whitespace (element : & NodeOrToken < ast :: TokenTree , syntax :: SyntaxToken >) -> bool { ! matches ! (element , NodeOrToken :: Token (token) if (token . kind () == SyntaxKind :: WHITESPACE)) }
    };
}

is_not_whitespace!()