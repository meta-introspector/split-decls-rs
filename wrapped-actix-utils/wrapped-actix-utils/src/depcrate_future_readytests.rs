// Generated macro for tests (module)
macro_rules! Depcrate_future_readytests {
() => {
// Module: crate::future::ready
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: rc :: Rc ; use futures_util :: task :: noop_waker ; use static_assertions :: { assert_impl_all , assert_not_impl_any } ; use super :: * ; assert_impl_all ! (Ready < () >: Send , Sync , Unpin , Clone) ; assert_impl_all ! (Ready < Rc < () >>: Unpin , Clone) ; assert_not_impl_any ! (Ready < Rc < () >>: Send , Sync) ; # [test] # [should_panic] fn multiple_poll_panics () { let waker = noop_waker () ; let mut cx = Context :: from_waker (& waker) ; let mut ready = ready (1) ; assert_eq ! (Pin :: new (& mut ready) . poll (& mut cx) , Poll :: Ready (1)) ; let _ = Pin :: new (& mut ready) . poll (& mut cx) ; } }
};
}
