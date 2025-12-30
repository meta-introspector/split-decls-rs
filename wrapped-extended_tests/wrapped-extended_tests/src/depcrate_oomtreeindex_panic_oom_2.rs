// Generated macro for treeindex_panic_oom_2 (function)
macro_rules! Depcrate_oomtreeindex_panic_oom_2 {
() => {
// Module: crate::oom
// Provides: {"treeindex_panic_oom_2"}
// Dependencies: {}
fn treeindex_panic_oom_2 (repeat : usize) { let treeindex : TreeIndex < usize , R > = TreeIndex :: default () ; for k in 0 .. repeat { assert ! (treeindex . insert_sync (k , R :: new (& INST_CNT , true)) . is_ok ()) ; } for k in 0 .. repeat { let result : Result < () , Box < dyn Any + Send > > = test_oom (| | { treeindex . remove_sync (& k) ; }) ; assert ! (result . is_err () || treeindex . peek_with (& k , | _ , _ | ()) . is_none ()) ; } drop (treeindex) ; }
};
}
