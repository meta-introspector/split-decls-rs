macro_rules! deps {
    () => {
        LocalReceiver!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < T > Stream for LocalReceiver < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut channel = self . channel . borrow_mut () ; match channel . queue . pop_front () { Some (item) => Poll :: Ready (Some (item)) , None => { if channel . closed { Poll :: Ready (None) } else { match & mut channel . waker { Some (prev) => prev . clone_from (cx . waker ()) , None => channel . waker = Some (cx . waker () . clone ()) , } Poll :: Pending } } } } }
    };
}

impl_118!()