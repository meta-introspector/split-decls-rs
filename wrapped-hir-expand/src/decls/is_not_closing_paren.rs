macro_rules! is_not_closing_paren {
    () => {
        fn is_not_closing_paren (element : & NodeOrToken < ast :: TokenTree , syntax :: SyntaxToken >) -> bool { ! matches ! (element , NodeOrToken :: Token (token) if (token . kind () == syntax :: T ! [')'])) }
    };
}

is_not_closing_paren!()