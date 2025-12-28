macro_rules! is_search_permitted_ancestors {
    () => {
        # [doc = " Returns whether we support matching within `node` and all of its ancestors."] fn is_search_permitted_ancestors (node : & SyntaxNode) -> bool { if let Some (parent) = node . parent () && ! is_search_permitted_ancestors (& parent) { return false ; } is_search_permitted (node) }
    };
}

is_search_permitted_ancestors!()