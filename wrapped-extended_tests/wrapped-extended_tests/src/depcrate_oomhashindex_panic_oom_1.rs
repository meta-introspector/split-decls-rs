// Generated macro for hashindex_panic_oom_1 (function)
macro_rules! Depcrate_oomhashindex_panic_oom_1 {
() => {
// Module: crate::oom
// Provides: {"hashindex_panic_oom_1"}
// Dependencies: {}
fn hashindex_panic_oom_1 (repeat : usize) { let hashindex : HashIndex < usize , R > = HashIndex :: default () ; for k in 0 .. repeat { let result : Result < () , Box < dyn Any + Send > > = test_oom (| | { hashindex . entry_sync (k) . or_insert_with (| | R :: new (& INST_CNT , true)) ; }) ; assert_eq ! (hashindex . peek_with (& k , | _ , _ | ()) . is_some () , result . is_ok ()) ; } drop (hashindex) ; }
};
}
