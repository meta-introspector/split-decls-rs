macro_rules! is_search_permitted {
    () => {
        # [doc = " Returns whether we support matching within this kind of node."] fn is_search_permitted (node : & SyntaxNode) -> bool { node . kind () != SyntaxKind :: USE }
    };
}

is_search_permitted!()