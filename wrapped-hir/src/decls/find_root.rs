macro_rules! find_root {
    () => {
        fn find_root (node : & SyntaxNode) -> SyntaxNode { node . ancestors () . last () . unwrap () }
    };
}

find_root!();