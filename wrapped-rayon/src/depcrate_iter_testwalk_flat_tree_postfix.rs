// Generated macro for walk_flat_tree_postfix (function)
macro_rules! Depcrate_iter_testwalk_flat_tree_postfix {
() => {
// Module: crate::iter::test
// Provides: {"walk_flat_tree_postfix"}
// Dependencies: {}
# [test] fn walk_flat_tree_postfix () { let v : Vec < _ > = crate :: iter :: walk_tree_postfix (99 , | & e | if e > 0 { Some (e - 1) } else { None }) . collect () ; assert ! (v . into_iter () . eq (0 .. 100)) ; }
};
}
