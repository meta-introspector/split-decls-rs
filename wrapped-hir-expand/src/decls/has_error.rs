macro_rules! has_error {
    () => {
        fn has_error (node : & SyntaxNode) -> bool { node . children () . any (| c | c . kind () == SyntaxKind :: ERROR) }
    };
}

has_error!()