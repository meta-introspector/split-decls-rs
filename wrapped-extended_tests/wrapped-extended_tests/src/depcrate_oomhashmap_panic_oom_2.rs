// Generated macro for hashmap_panic_oom_2 (function)
macro_rules! Depcrate_oomhashmap_panic_oom_2 {
() => {
// Module: crate::oom
// Provides: {"hashmap_panic_oom_2"}
// Dependencies: {}
fn hashmap_panic_oom_2 (repeat : usize) { let hashmap : HashMap < usize , R > = HashMap :: default () ; for k in 0 .. repeat { let result : Result < () , Box < dyn Any + Send > > = test_oom (| | { let scc :: hash_map :: Entry :: < usize , R > :: Vacant (entry) = hashmap . entry_sync (k) else { return ; } ; entry . insert_entry (R :: new (& INST_CNT , true)) ; }) ; assert_eq ! (hashmap . read_sync (& k , | _ , _ | ()) . is_some () , result . is_ok ()) ; } drop (hashmap) ; }
};
}
