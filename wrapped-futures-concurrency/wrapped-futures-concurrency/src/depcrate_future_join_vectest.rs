// Generated macro for test (module)
macro_rules! Depcrate_future_join_vectest {
() => {
// Module: crate::future::join::vec
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: utils :: DummyWaker ; use alloc :: format ; use alloc :: sync :: Arc ; use alloc :: vec ; use core :: future ; # [test] fn smoke () { futures_lite :: future :: block_on (async { let fut = vec ! [future :: ready ("hello") , future :: ready ("world")] . join () ; assert_eq ! (fut . await , vec ! ["hello" , "world"]) ; }) ; } # [test] fn empty () { futures_lite :: future :: block_on (async { let data : Vec < future :: Ready < () > > = vec ! [] ; let fut = data . join () ; assert_eq ! (fut . await , vec ! []) ; }) ; } # [test] fn debug () { let mut fut = vec ! [future :: ready ("hello") , future :: ready ("world")] . join () ; assert_eq ! (format ! ("{fut:?}") , "[Pending, Pending]") ; let mut fut = Pin :: new (& mut fut) ; let waker = Arc :: new (DummyWaker ()) . into () ; let mut cx = Context :: from_waker (& waker) ; let _ = fut . as_mut () . poll (& mut cx) ; assert_eq ! (format ! ("{fut:?}") , "[None, None]") ; } }
};
}
