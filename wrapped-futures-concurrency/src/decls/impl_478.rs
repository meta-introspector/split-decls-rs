macro_rules! deps {
    () => {
        Zip!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl < S , const N : usize > Stream for Zip < S , N > where S : Stream , { type Item = [S :: Item ; N] ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; assert ! (!* this . done , "Stream should not be polled after completion") ; let mut readiness = this . wakers . readiness () ; readiness . set_waker (cx . waker ()) ; for index in 0 .. N { if ! readiness . any_ready () { return Poll :: Pending ; } else if this . state [index] . is_ready () || ! readiness . clear_ready (index) { continue ; } # [allow (clippy :: drop_non_drop)] drop (readiness) ; let mut cx = Context :: from_waker (this . wakers . get (index) . unwrap ()) ; let stream = utils :: get_pin_mut (this . streams . as_mut () , index) . unwrap () ; match stream . poll_next (& mut cx) { Poll :: Ready (Some (item)) => { this . output [index] = MaybeUninit :: new (item) ; this . state [index] . set_ready () ; let all_ready = this . state . iter () . all (| state | state . is_ready ()) ; if all_ready { readiness = this . wakers . readiness () ; readiness . set_all_ready () ; this . state . set_all_pending () ; let mut output = array :: from_fn (| _ | MaybeUninit :: uninit ()) ; mem :: swap (this . output , & mut output) ; let output = unsafe { array_assume_init (output) } ; return Poll :: Ready (Some (output)) ; } } Poll :: Ready (None) => { * this . done = true ; return Poll :: Ready (None) ; } Poll :: Pending => { } } readiness = this . wakers . readiness () ; } Poll :: Pending } }
    };
}

impl_478!()