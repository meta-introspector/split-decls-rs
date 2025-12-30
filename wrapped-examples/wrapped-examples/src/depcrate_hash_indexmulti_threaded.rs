// Generated macro for multi_threaded (function)
macro_rules! Depcrate_hash_indexmulti_threaded {
() => {
// Module: crate::hash_index
// Provides: {"multi_threaded"}
// Dependencies: {}
# [test] fn multi_threaded () { let workload_size = 256 ; let hashindex : Arc < HashIndex < isize , isize > > = Arc :: default () ; thread :: scope (| s | { s . spawn (| | { for i in 1 .. workload_size { assert ! (hashindex . insert_sync (i , i) . is_ok ()) ; } assert ! (hashindex . peek_with (& 0 , | _ , _ | ()) . is_none ()) ; for i in 1 .. workload_size { assert ! (hashindex . peek_with (& i , | _ , _ | ()) . is_some ()) ; } for i in 1 .. workload_size { assert ! (hashindex . remove_sync (& i)) ; } }) ; s . spawn (| | { for i in 1 .. workload_size { assert ! (hashindex . insert_sync (- i , i) . is_ok ()) ; } assert ! (hashindex . peek_with (& 0 , | _ , _ | ()) . is_none ()) ; for i in 1 .. workload_size { assert ! (hashindex . peek_with (&- i , | _ , _ | ()) . is_some ()) ; } for i in 1 .. workload_size { assert ! (hashindex . remove_sync (&- i)) ; } }) ; }) ; assert ! (hashindex . is_empty ()) ; }
};
}
