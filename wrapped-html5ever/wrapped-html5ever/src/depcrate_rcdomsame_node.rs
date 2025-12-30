// Generated macro for same_node (function)
macro_rules! Depcrate_rcdomsame_node {
() => {
// Module: crate::rcdom
// Provides: {"same_node"}
// Dependencies: {}
# [allow (trivial_casts)] fn same_node (x : & Handle , y : & Handle) -> bool { (& * x . borrow () as * const Node) == (& * y . borrow () as * const Node) }
};
}
