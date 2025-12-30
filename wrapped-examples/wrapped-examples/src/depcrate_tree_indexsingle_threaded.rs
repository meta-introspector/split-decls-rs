// Generated macro for single_threaded (function)
macro_rules! Depcrate_tree_indexsingle_threaded {
() => {
// Module: crate::tree_index
// Provides: {"single_threaded"}
// Dependencies: {}
# [test] fn single_threaded () { let workload_size = 256 ; let treeindex : TreeIndex < isize , isize > = TreeIndex :: new () ; for i in 1 .. workload_size { if i % 2 == 0 { assert ! (treeindex . insert_sync (- i , i) . is_ok ()) ; } else { assert ! (treeindex . insert_sync (i , i) . is_ok ()) ; } } for i in 1 .. workload_size { if i % 2 == 0 { assert ! (treeindex . peek_with (& i , | _ , _ | ()) . is_none ()) ; assert ! (treeindex . peek_with (&- i , | _ , _ | ()) . is_some ()) ; } else { assert ! (treeindex . peek_with (& i , | _ , _ | ()) . is_some ()) ; assert ! (treeindex . peek_with (&- i , | _ , _ | ()) . is_none ()) ; } } for i in 1 .. workload_size { if i % 2 == 0 { assert ! (! treeindex . remove_sync (& i)) ; assert ! (treeindex . remove_sync (&- i)) ; assert ! (! treeindex . remove_sync (&- i)) ; } else { assert ! (! treeindex . remove_sync (&- i)) ; assert ! (treeindex . remove_sync (& i)) ; assert ! (! treeindex . remove_sync (& i)) ; } } assert ! (treeindex . is_empty ()) ; }
};
}
