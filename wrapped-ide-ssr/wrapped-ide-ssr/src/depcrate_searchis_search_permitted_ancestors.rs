// Generated macro for is_search_permitted_ancestors (function)
macro_rules! Depcrate_searchis_search_permitted_ancestors {
() => {
// Module: crate::search
// Provides: {"is_search_permitted_ancestors"}
// Dependencies: {}
# [doc = " Returns whether we support matching within `node` and all of its ancestors."] fn is_search_permitted_ancestors (node : & SyntaxNode) -> bool { if let Some (parent) = node . parent () && ! is_search_permitted_ancestors (& parent) { return false ; } is_search_permitted (node) }
};
}
