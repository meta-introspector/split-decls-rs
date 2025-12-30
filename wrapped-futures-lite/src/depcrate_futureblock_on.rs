// Generated macro for block_on (function)
macro_rules! Depcrate_futureblock_on {
() => {
// Module: crate::future
// Provides: {"block_on"}
// Dependencies: {}
# [doc = " Blocks the current thread on a future."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future;"] # [doc = ""] # [doc = " let val = future::block_on(async {"] # [doc = "     1 + 2"] # [doc = " });"] # [doc = ""] # [doc = " assert_eq!(val, 3);"] # [doc = " ```"] # [cfg (feature = "std")] pub fn block_on < T > (future : impl Future < Output = T >) -> T { use core :: cell :: RefCell ; use core :: task :: Waker ; use parking :: Parker ; crate :: pin ! (future) ; fn parker_and_waker () -> (Parker , Waker) { let parker = Parker :: new () ; let unparker = parker . unparker () ; let waker = Waker :: from (unparker) ; (parker , waker) } thread_local ! { static CACHE : RefCell < (Parker , Waker) > = RefCell :: new (parker_and_waker ()) ; } CACHE . with (| cache | { let tmp_cached ; let tmp_fresh ; let (parker , waker) = match cache . try_borrow_mut () { Ok (cache) => { tmp_cached = cache ; & * tmp_cached } Err (_) => { tmp_fresh = parker_and_waker () ; & tmp_fresh } } ; let cx = & mut Context :: from_waker (waker) ; loop { match future . as_mut () . poll (cx) { Poll :: Ready (output) => return output , Poll :: Pending => parker . park () , } } }) }
};
}
