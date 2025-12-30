// Generated macro for single_threaded (function)
macro_rules! Depcrate_hash_mapsingle_threaded {
() => {
// Module: crate::hash_map
// Provides: {"single_threaded"}
// Dependencies: {}
# [test] fn single_threaded () { let workload_size = 256 ; let hashmap : HashMap < isize , isize > = HashMap :: new () ; for i in 1 .. workload_size { if i % 2 == 0 { assert ! (hashmap . insert_sync (- i , i) . is_ok ()) ; } else { assert ! (hashmap . insert_sync (i , i) . is_ok ()) ; } } for i in 1 .. workload_size { if i % 2 == 0 { assert ! (hashmap . get_sync (& i) . is_none ()) ; assert ! (hashmap . get_sync (&- i) . is_some ()) ; } else { assert ! (hashmap . get_sync (& i) . is_some ()) ; assert ! (hashmap . get_sync (&- i) . is_none ()) ; } } for i in 1 .. workload_size { if i % 2 == 0 { assert ! (hashmap . remove_sync (& i) . is_none ()) ; assert ! (hashmap . remove_sync (&- i) . is_some ()) ; assert ! (hashmap . remove_sync (&- i) . is_none ()) ; } else { assert ! (hashmap . remove_sync (&- i) . is_none ()) ; assert ! (hashmap . remove_sync (& i) . is_some ()) ; assert ! (hashmap . remove_sync (& i) . is_none ()) ; } } assert ! (hashmap . is_empty ()) ; }
};
}
