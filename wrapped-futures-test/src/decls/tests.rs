macro_rules! deps {
    () => {
        AssertUnmoved!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use futures_core :: future :: Future ; use futures_core :: task :: { Context , Poll } ; use futures_util :: future :: pending ; use futures_util :: task :: noop_waker ; use std :: pin :: Pin ; use super :: AssertUnmoved ; # [test] fn assert_send_sync () { fn assert < T : Send + Sync > () { } assert :: < AssertUnmoved < () > > () ; } # [test] fn dont_panic_when_not_polled () { let future = AssertUnmoved :: new (pending :: < () > ()) ; drop (future) ; } # [test] # [should_panic (expected = "AssertUnmoved moved between poll calls")] fn dont_double_panic () { let waker = noop_waker () ; let mut cx = Context :: from_waker (& waker) ; let mut future = AssertUnmoved :: new (pending :: < () > ()) ; let pinned_future = unsafe { Pin :: new_unchecked (& mut future) } ; assert_eq ! (pinned_future . poll (& mut cx) , Poll :: Pending) ; let mut future = Box :: new (future) ; let pinned_boxed_future = unsafe { Pin :: new_unchecked (& mut * future) } ; assert_eq ! (pinned_boxed_future . poll (& mut cx) , Poll :: Pending) ; } }
    };
}

tests!();