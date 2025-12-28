macro_rules! deps {
    () => {
        Join!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl < Fut > Future for Join < Fut > where Fut : Future , { type Output = Vec < Fut :: Output > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; assert ! (!* this . consumed , "Futures must not be polled after completing") ; let mut readiness = this . wakers . readiness () ; readiness . set_waker (cx . waker ()) ; if * this . pending != 0 && ! readiness . any_ready () { return Poll :: Pending ; } let futures = this . futures . as_mut () ; let states = & mut this . state [..] ; for (i , mut fut) in futures . iter () . enumerate () { if states [i] . is_pending () && readiness . clear_ready (i) { # [allow (clippy :: drop_non_drop)] drop (readiness) ; let mut cx = Context :: from_waker (this . wakers . get (i) . unwrap ()) ; if let Poll :: Ready (value) = unsafe { fut . as_mut () . map_unchecked_mut (| t | t . deref_mut ()) . poll (& mut cx) } { this . items . write (i , value) ; states [i] . set_ready () ; * this . pending -= 1 ; unsafe { ManuallyDrop :: drop (fut . get_unchecked_mut ()) } ; } readiness = this . wakers . readiness () ; } } if * this . pending == 0 { * this . consumed = true ; this . state . iter_mut () . for_each (| state | { debug_assert ! (state . is_ready () , "Future should have reached a `Ready` state") ; state . set_none () ; }) ; Poll :: Ready (unsafe { this . items . take () }) } else { Poll :: Pending } } }
    };
}

impl_251!()