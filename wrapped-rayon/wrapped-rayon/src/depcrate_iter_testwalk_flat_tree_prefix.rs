// Generated macro for walk_flat_tree_prefix (function)
macro_rules! Depcrate_iter_testwalk_flat_tree_prefix {
() => {
// Module: crate::iter::test
// Provides: {"walk_flat_tree_prefix"}
// Dependencies: {}
# [test] fn walk_flat_tree_prefix () { let v : Vec < _ > = crate :: iter :: walk_tree_prefix (0 , | & e | if e < 99 { Some (e + 1) } else { None }) . collect () ; assert ! (v . into_iter () . eq (0 .. 100)) ; }
};
}
