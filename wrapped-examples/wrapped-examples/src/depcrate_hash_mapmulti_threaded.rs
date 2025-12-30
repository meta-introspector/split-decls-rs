// Generated macro for multi_threaded (function)
macro_rules! Depcrate_hash_mapmulti_threaded {
() => {
// Module: crate::hash_map
// Provides: {"multi_threaded"}
// Dependencies: {}
# [test] fn multi_threaded () { let workload_size = 256 ; let hashmap : Arc < HashMap < isize , isize > > = Arc :: default () ; thread :: scope (| s | { s . spawn (| | { for i in 1 .. workload_size { assert ! (hashmap . insert_sync (i , i) . is_ok ()) ; } assert ! (hashmap . get_sync (& 0) . is_none ()) ; for i in 1 .. workload_size { assert ! (hashmap . get_sync (& i) . is_some ()) ; } for i in 1 .. workload_size { assert ! (hashmap . remove_sync (& i) . is_some ()) ; } }) ; s . spawn (| | { for i in 1 .. workload_size { assert ! (hashmap . insert_sync (- i , i) . is_ok ()) ; } assert ! (hashmap . get_sync (& 0) . is_none ()) ; for i in 1 .. workload_size { assert ! (hashmap . get_sync (&- i) . is_some ()) ; } for i in 1 .. workload_size { assert ! (hashmap . remove_sync (&- i) . is_some ()) ; } }) ; }) ; assert ! (hashmap . is_empty ()) ; }
};
}
