// Generated macro for create_deep_object (function)
macro_rules! Depcrate_nestedcreate_deep_object {
() => {
// Module: crate::nested
// Provides: {"create_deep_object"}
// Dependencies: {}
# [doc = " Create a deep object with the given amount of nodes."] pub fn create_deep_object < W : std :: io :: Write > (writer : & mut W , n_nodes : usize ,) -> std :: io :: Result < () > { let mut tree = Tree :: new () ; for _ in 0 .. n_nodes { tree . push_node () ; } tree . write_to (writer) }
};
}
