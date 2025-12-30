// Generated macro for impl_198 (impl)
macro_rules! Depcrate_futureimpl_198 {
() => {
// Module: crate::future
// Provides: {"impl_198"}
// Dependencies: {}
impl < A : Async > Future for AsyncFuture < A > { type Output = Result < A :: Output > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { if self . status . Status () ? != AsyncStatus :: Started { return Poll :: Ready (self . inner . get_results ()) ; } if let Some (shared_waker) = & self . waker { let mut guard = shared_waker . lock () . unwrap () ; guard . clone_from (cx . waker ()) ; if self . status . Status () ? != AsyncStatus :: Started { return Poll :: Ready (self . inner . get_results ()) ; } } else { let shared_waker = Arc :: new (Mutex :: new (cx . waker () . clone ())) ; self . waker = Some (shared_waker . clone ()) ; self . inner . set_completed (move | _ | { shared_waker . lock () . unwrap () . wake_by_ref () ; }) ? ; } ; Poll :: Pending } }
};
}
