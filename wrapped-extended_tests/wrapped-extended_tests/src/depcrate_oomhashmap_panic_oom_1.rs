// Generated macro for hashmap_panic_oom_1 (function)
macro_rules! Depcrate_oomhashmap_panic_oom_1 {
() => {
// Module: crate::oom
// Provides: {"hashmap_panic_oom_1"}
// Dependencies: {}
fn hashmap_panic_oom_1 (repeat : usize) { let hashmap : HashMap < usize , R > = HashMap :: default () ; for k in 0 .. repeat { let result : Result < () , Box < dyn Any + Send > > = test_oom (| | { hashmap . entry_sync (k) . or_insert_with (| | R :: new (& INST_CNT , true)) ; }) ; assert_eq ! (hashmap . read_sync (& k , | _ , _ | ()) . is_some () , result . is_ok ()) ; } drop (hashmap) ; }
};
}
