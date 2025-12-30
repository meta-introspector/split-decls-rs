// Generated macro for opt_as_use_node (function)
macro_rules! Depcrate_min_ident_charsopt_as_use_node {
() => {
// Module: crate::min_ident_chars
// Provides: {"opt_as_use_node"}
// Dependencies: {}
# [doc = " Attempt to convert the node to an [`ItemKind::Use`] node."] # [doc = ""] # [doc = " If it is, return the [`UsePath`] contained within."] fn opt_as_use_node (node : Node < '_ >) -> Option < & '_ UsePath < '_ > > { if let Node :: Item (item) = node && let ItemKind :: Use (path , _) = item . kind { Some (path) } else { None } }
};
}
