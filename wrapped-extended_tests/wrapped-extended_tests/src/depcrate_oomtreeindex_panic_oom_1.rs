// Generated macro for treeindex_panic_oom_1 (function)
macro_rules! Depcrate_oomtreeindex_panic_oom_1 {
() => {
// Module: crate::oom
// Provides: {"treeindex_panic_oom_1"}
// Dependencies: {}
fn treeindex_panic_oom_1 (repeat : usize) { PANIC_COUNT . store (0 , Relaxed) ; let treeindex : TreeIndex < usize , R > = TreeIndex :: default () ; for k in 0 .. repeat { let result : Result < () , Box < dyn Any + Send > > = test_oom (| | { assert ! (treeindex . insert_sync (k , R :: new (& INST_CNT , true)) . is_ok ()) ; }) ; assert_eq ! (treeindex . peek_with (& k , | _ , _ | ()) . is_some () , result . is_ok ()) ; } drop (treeindex) ; }
};
}
