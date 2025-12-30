// Generated macro for impl_42 (impl)
macro_rules! Depcrate_join_allimpl_42 {
() => {
// Module: crate::join_all
// Provides: {"impl_42"}
// Dependencies: {}
impl < T > Future for JoinAll < T > { type Output = Vec < T > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut ready = true ; let this = self . get_mut () ; for fut in this . fut . iter_mut () { if let JoinFuture :: Future (f) = fut { match f . as_mut () . poll (cx) { Poll :: Ready (t) => { * fut = JoinFuture :: Result (Some (t)) ; } Poll :: Pending => ready = false , } } } if ready { let mut res = Vec :: new () ; for fut in this . fut . iter_mut () { if let JoinFuture :: Result (f) = fut { res . push (f . take () . unwrap ()) ; } } Poll :: Ready (res) } else { Poll :: Pending } } }
};
}
