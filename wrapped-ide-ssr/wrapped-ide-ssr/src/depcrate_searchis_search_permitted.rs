// Generated macro for is_search_permitted (function)
macro_rules! Depcrate_searchis_search_permitted {
() => {
// Module: crate::search
// Provides: {"is_search_permitted"}
// Dependencies: {}
# [doc = " Returns whether we support matching within this kind of node."] fn is_search_permitted (node : & SyntaxNode) -> bool { node . kind () != SyntaxKind :: USE }
};
}
