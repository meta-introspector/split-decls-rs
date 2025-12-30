// Generated macro for walk_tree_postfix (function)
macro_rules! Depcrate_iter_testwalk_tree_postfix {
() => {
// Module: crate::iter::test
// Provides: {"walk_tree_postfix"}
// Dependencies: {}
# [test] fn walk_tree_postfix () { let v : Vec < _ > = crate :: iter :: walk_tree_postfix (0u64 .. 100 , | r | { let mid = (r . start + r . end - 1) / 2 ; std :: iter :: once (r . start .. mid) . chain (std :: iter :: once (mid .. (r . end - 1))) . filter (| r | ! r . is_empty ()) }) . map (| r | r . end - 1) . collect () ; assert ! (v . into_iter () . eq (0 .. 100)) ; }
};
}
