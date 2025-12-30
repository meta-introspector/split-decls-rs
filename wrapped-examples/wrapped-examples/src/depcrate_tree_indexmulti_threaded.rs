// Generated macro for multi_threaded (function)
macro_rules! Depcrate_tree_indexmulti_threaded {
() => {
// Module: crate::tree_index
// Provides: {"multi_threaded"}
// Dependencies: {}
# [test] fn multi_threaded () { let workload_size = 256 ; let treeindex : Arc < TreeIndex < isize , isize > > = Arc :: default () ; thread :: scope (| s | { s . spawn (| | { for i in 1 .. workload_size { assert ! (treeindex . insert_sync (i , i) . is_ok ()) ; } assert ! (treeindex . peek_with (& 0 , | _ , _ | ()) . is_none ()) ; for i in 1 .. workload_size { assert ! (treeindex . peek_with (& i , | _ , _ | ()) . is_some ()) ; } for i in 1 .. workload_size { assert ! (treeindex . remove_sync (& i)) ; } }) ; s . spawn (| | { for i in 1 .. workload_size { assert ! (treeindex . insert_sync (- i , i) . is_ok ()) ; } assert ! (treeindex . peek_with (& 0 , | _ , _ | ()) . is_none ()) ; for i in 1 .. workload_size { assert ! (treeindex . peek_with (&- i , | _ , _ | ()) . is_some ()) ; } for i in 1 .. workload_size { assert ! (treeindex . remove_sync (&- i)) ; } }) ; }) ; assert ! (treeindex . is_empty ()) ; }
};
}
