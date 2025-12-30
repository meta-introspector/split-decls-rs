// Generated macro for impl_439 (impl)
macro_rules! Depcrate_future_join_arrayimpl_439 {
() => {
// Module: crate::future::join::array
// Provides: {"impl_439"}
// Dependencies: {}
impl < Fut , const N : usize > Future for Join < Fut , N > where Fut : Future , { type Output = [Fut :: Output ; N] ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; assert ! (!* this . consumed , "Futures must not be polled after completing") ; let mut readiness = this . wakers . readiness () ; readiness . set_waker (cx . waker ()) ; if * this . pending != 0 && ! readiness . any_ready () { return Poll :: Pending ; } for (i , mut fut) in this . futures . iter () . enumerate () { if this . state [i] . is_pending () && readiness . clear_ready (i) { # [allow (clippy :: drop_non_drop)] drop (readiness) ; let mut cx = Context :: from_waker (this . wakers . get (i) . unwrap ()) ; if let Poll :: Ready (value) = unsafe { fut . as_mut () . map_unchecked_mut (| t | t . deref_mut ()) . poll (& mut cx) } { this . items . write (i , value) ; this . state [i] . set_ready () ; * this . pending -= 1 ; unsafe { ManuallyDrop :: drop (fut . get_unchecked_mut ()) } ; } readiness = this . wakers . readiness () ; } } if * this . pending == 0 { * this . consumed = true ; for state in this . state . iter_mut () { debug_assert ! (state . is_ready () , "Future should have reached a `Ready` state") ; state . set_none () ; } Poll :: Ready (unsafe { this . items . take () }) } else { Poll :: Pending } } }
};
}
