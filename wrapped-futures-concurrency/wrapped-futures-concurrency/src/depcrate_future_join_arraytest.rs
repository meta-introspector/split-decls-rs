// Generated macro for test (module)
macro_rules! Depcrate_future_join_arraytest {
() => {
// Module: crate::future::join::array
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use core :: future ; # [test] fn smoke () { futures_lite :: future :: block_on (async { let fut = [future :: ready ("hello") , future :: ready ("world")] . join () ; assert_eq ! (fut . await , ["hello" , "world"]) ; }) ; } # [test] fn empty () { futures_lite :: future :: block_on (async { let data : [future :: Ready < () > ; 0] = [] ; let fut = data . join () ; assert_eq ! (fut . await , []) ; }) ; } # [test] # [cfg (feature = "alloc")] fn debug () { use crate :: utils :: DummyWaker ; use alloc :: format ; use alloc :: sync :: Arc ; use core :: task :: Context ; let mut fut = [future :: ready ("hello") , future :: ready ("world")] . join () ; assert_eq ! (format ! ("{fut:?}") , "[Pending, Pending]") ; let mut fut = Pin :: new (& mut fut) ; let waker = Arc :: new (DummyWaker ()) . into () ; let mut cx = Context :: from_waker (& waker) ; let _ = fut . as_mut () . poll (& mut cx) ; assert_eq ! (format ! ("{fut:?}") , "[None, None]") ; } }
};
}
