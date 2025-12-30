// Generated macro for walk_tree_postfix_degree5 (function)
macro_rules! Depcrate_iter_testwalk_tree_postfix_degree5 {
() => {
// Module: crate::iter::test
// Provides: {"walk_tree_postfix_degree5"}
// Dependencies: {}
# [test] fn walk_tree_postfix_degree5 () { let depth = 5 ; let nodes_number = (1 - 5i32 . pow (depth)) / (1 - 5) ; let nodes = (0 .. nodes_number) . collect :: < Vec < _ > > () ; let v : Vec < i32 > = crate :: iter :: walk_tree_postfix (nodes . as_slice () , | & r | { r . split_last () . into_iter () . filter_map (| (_ , r) | if r . is_empty () { None } else { Some (r) }) . flat_map (| r | r . chunks (r . len () / 5)) }) . filter_map (| r | r . last () . copied ()) . collect () ; assert_eq ! (v , nodes) }
};
}
