// Generated macro for hashindex_panic_oom_2 (function)
macro_rules! Depcrate_oomhashindex_panic_oom_2 {
() => {
// Module: crate::oom
// Provides: {"hashindex_panic_oom_2"}
// Dependencies: {}
fn hashindex_panic_oom_2 (repeat : usize) { let hashindex : HashIndex < usize , R > = HashIndex :: default () ; for k in 0 .. repeat { let result : Result < () , Box < dyn Any + Send > > = test_oom (| | { let scc :: hash_index :: Entry :: < usize , R > :: Vacant (entry) = hashindex . entry_sync (k) else { return ; } ; entry . insert_entry (R :: new (& INST_CNT , true)) ; }) ; assert_eq ! (hashindex . peek_with (& k , | _ , _ | ()) . is_some () , result . is_ok ()) ; } drop (hashindex) ; }
};
}
