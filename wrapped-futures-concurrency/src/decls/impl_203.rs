macro_rules! deps {
    () => {
        PollState!();
        Key!();
        FutureGroup!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < F : Future > FutureGroup < F > { fn poll_next_inner (self : Pin < & mut Self > , cx : & Context < '_ > ,) -> Poll < Option < (Key , < F as Future > :: Output) > > { let mut this = self . project () ; if this . futures . is_empty () { return Poll :: Ready (None) ; } let mut readiness = this . wakers . readiness () ; readiness . set_waker (cx . waker ()) ; if ! readiness . any_ready () { return Poll :: Pending ; } let mut ret = Poll :: Pending ; let states = this . states ; let futures = unsafe { this . futures . as_mut () . get_unchecked_mut () } ; for index in this . keys . iter () . cloned () { if states [index] . is_pending () && readiness . clear_ready (index) { # [allow (clippy :: drop_non_drop)] drop (readiness) ; let mut cx = Context :: from_waker (this . wakers . get (index) . unwrap ()) ; let future = unsafe { Pin :: new_unchecked (& mut futures [index]) } ; match future . poll (& mut cx) { Poll :: Ready (item) => { ret = Poll :: Ready (Some ((Key (index) , item))) ; states [index] = PollState :: None ; futures . remove (index) ; break ; } Poll :: Pending => { } } ; readiness = this . wakers . readiness () ; } } if let Poll :: Ready (Some ((key , _))) = ret { this . keys . remove (& key . 0) ; } ret } }
    };
}

impl_203!();