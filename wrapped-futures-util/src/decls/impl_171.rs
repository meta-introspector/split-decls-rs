macro_rules! deps {
    () => {
        Ready!();
        TryMaybeDone!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < Fut : TryFuture > Future for TryMaybeDone < Fut > { type Output = Result < () , Fut :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { unsafe { match self . as_mut () . get_unchecked_mut () { Self :: Future (f) => match ready ! (Pin :: new_unchecked (f) . try_poll (cx)) { Ok (res) => self . set (Self :: Done (res)) , Err (e) => { self . set (Self :: Gone) ; return Poll :: Ready (Err (e)) ; } } , Self :: Done (_) => { } Self :: Gone => panic ! ("TryMaybeDone polled after value taken") , } } Poll :: Ready (Ok (())) } }
    };
}

impl_171!();