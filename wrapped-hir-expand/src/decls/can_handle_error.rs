macro_rules! can_handle_error {
    () => {
        fn can_handle_error (node : & SyntaxNode) -> bool { ast :: Expr :: can_cast (node . kind ()) }
    };
}

can_handle_error!();