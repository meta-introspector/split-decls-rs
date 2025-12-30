// Generated macro for walk_tree_prefix (function)
macro_rules! Depcrate_iter_testwalk_tree_prefix {
() => {
// Module: crate::iter::test
// Provides: {"walk_tree_prefix"}
// Dependencies: {}
# [test] fn walk_tree_prefix () { let v : Vec < u32 > = crate :: iter :: walk_tree_prefix (0u32 .. 100 , | r | { let mid = (r . start + 1 + r . end) / 2 ; std :: iter :: once ((r . start + 1) .. mid) . chain (std :: iter :: once (mid .. r . end)) . filter (| r | ! r . is_empty ()) }) . map (| r | r . start) . collect () ; assert ! (v . into_iter () . eq (0 .. 100)) ; }
};
}
