// Generated macro for single_threaded (function)
macro_rules! Depcrate_hash_indexsingle_threaded {
() => {
// Module: crate::hash_index
// Provides: {"single_threaded"}
// Dependencies: {}
# [test] fn single_threaded () { let workload_size = 256 ; let hashindex : HashIndex < isize , isize > = HashIndex :: new () ; for i in 1 .. workload_size { if i % 2 == 0 { assert ! (hashindex . insert_sync (- i , i) . is_ok ()) ; } else { assert ! (hashindex . insert_sync (i , i) . is_ok ()) ; } } for i in 1 .. workload_size { if i % 2 == 0 { assert ! (hashindex . peek_with (& i , | _ , _ | ()) . is_none ()) ; assert ! (hashindex . peek_with (&- i , | _ , _ | ()) . is_some ()) ; } else { assert ! (hashindex . peek_with (& i , | _ , _ | ()) . is_some ()) ; assert ! (hashindex . peek_with (&- i , | _ , _ | ()) . is_none ()) ; } } for i in 1 .. workload_size { if i % 2 == 0 { assert ! (! hashindex . remove_sync (& i)) ; assert ! (hashindex . remove_sync (&- i)) ; assert ! (! hashindex . remove_sync (&- i)) ; } else { assert ! (! hashindex . remove_sync (&- i)) ; assert ! (hashindex . remove_sync (& i)) ; assert ! (! hashindex . remove_sync (& i)) ; } } assert ! (hashindex . is_empty ()) ; }
};
}
