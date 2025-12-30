// Generated macro for tests (module)
macro_rules! Depcrate_future_poll_fntests {
() => {
// Module: crate::future::poll_fn
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: marker :: PhantomPinned ; use super :: * ; static_assertions :: assert_impl_all ! (PollFn < () >: Unpin) ; static_assertions :: assert_not_impl_all ! (PollFn < PhantomPinned >: Unpin) ; # [actix_rt :: test] async fn test_poll_fn () { let res = poll_fn (| _ | Poll :: Ready (42)) . await ; assert_eq ! (res , 42) ; let mut i = 5 ; let res = poll_fn (| cx | { i -= 1 ; if i > 0 { cx . waker () . wake_by_ref () ; Poll :: Pending } else { Poll :: Ready (42) } }) . await ; assert_eq ! (res , 42) ; } # [allow (dead_code)] fn require_send < T : Send > (_t : & T) { } # [allow (dead_code)] fn require_sync < T : Sync > (_t : & T) { } # [allow (unused)] trait AmbiguousIfUnpin < A > { fn some_item (& self) { } } impl < T : ? Sized > AmbiguousIfUnpin < () > for T { } impl < T : ? Sized + Unpin > AmbiguousIfUnpin < [u8 ; 0] > for T { } const _ : fn () = | | { let pinned = std :: marker :: PhantomPinned ; let f = poll_fn (move | _ | { let _ = & pinned ; std :: task :: Poll :: Pending :: < () > }) ; require_send (& f) ; require_sync (& f) ; AmbiguousIfUnpin :: some_item (& f) ; } ; }
};
}
