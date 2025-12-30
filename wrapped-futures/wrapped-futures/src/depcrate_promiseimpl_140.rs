// Generated macro for impl_140 (impl)
macro_rules! Depcrate_promiseimpl_140 {
() => {
// Module: crate::promise
// Provides: {"impl_140"}
// Dependencies: {}
impl < T : Send + 'static > Future for Promise < T > { type Item = T ; type Error = Canceled ; fn poll (& mut self , _ : & mut Task) -> Poll < T , Canceled > { if self . inner . pending_wake . load (Ordering :: SeqCst) { return Poll :: NotReady } match self . inner . slot . try_consume () { Ok (Some (e)) => Poll :: Ok (e) , Ok (None) => Poll :: Err (Canceled) , Err (_) => Poll :: NotReady , } } fn schedule (& mut self , task : & mut Task) { if self . inner . pending_wake . load (Ordering :: SeqCst) { if let Some (cancel_token) = self . cancel_token . take () { self . inner . slot . cancel (cancel_token) ; } } self . inner . pending_wake . store (true , Ordering :: SeqCst) ; let inner = self . inner . clone () ; let handle = task . handle () . clone () ; self . cancel_token = Some (self . inner . slot . on_full (move | _ | { inner . pending_wake . store (false , Ordering :: SeqCst) ; handle . notify () ; })) ; } }
};
}
