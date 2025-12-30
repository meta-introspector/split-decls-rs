// Generated macro for test (module)
macro_rules! Depcrate_future_join_tupletest {
() => {
// Module: crate::future::join::tuple
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use core :: future ; # [test] # [allow (clippy :: unit_cmp)] fn join_0 () { futures_lite :: future :: block_on (async { assert_eq ! (() . join () . await , ()) ; }) ; } # [test] fn join_1 () { futures_lite :: future :: block_on (async { let a = future :: ready ("hello") ; assert_eq ! ((a ,) . join () . await , ("hello" ,)) ; }) ; } # [test] fn join_2 () { futures_lite :: future :: block_on (async { let a = future :: ready ("hello") ; let b = future :: ready (12) ; assert_eq ! ((a , b) . join () . await , ("hello" , 12)) ; }) ; } # [test] fn join_3 () { futures_lite :: future :: block_on (async { let a = future :: ready ("hello") ; let b = future :: ready ("world") ; let c = future :: ready (12) ; assert_eq ! ((a , b , c) . join () . await , ("hello" , "world" , 12)) ; }) ; } # [test] # [cfg (feature = "std")] fn does_not_leak_memory () { use core :: cell :: RefCell ; use futures_lite :: future :: pending ; thread_local ! { static NOT_LEAKING : RefCell < bool > = const { RefCell :: new (false) } ; } ; struct FlipFlagAtDrop ; impl Drop for FlipFlagAtDrop { fn drop (& mut self) { NOT_LEAKING . with (| v | { * v . borrow_mut () = true ; }) ; } } futures_lite :: future :: block_on (async { let string = future :: ready ("memory leak" . to_owned ()) ; let flip = future :: ready (FlipFlagAtDrop) ; let leak = (string , flip , pending :: < u8 > ()) . join () ; _ = futures_lite :: future :: poll_once (leak) . await ; }) ; NOT_LEAKING . with (| flag | { assert ! (* flag . borrow ()) ; }) } }
};
}
