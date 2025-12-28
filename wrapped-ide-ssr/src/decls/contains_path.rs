macro_rules! contains_path {
    () => {
        # [doc = " Returns whether there are any paths in `node`."] fn contains_path (node : & SyntaxNode) -> bool { node . kind () == SyntaxKind :: PATH || node . descendants () . any (| node | node . kind () == SyntaxKind :: PATH) }
    };
}

contains_path!();