macro_rules! deps {
    () => {
        MaybeDone!();
        Ready!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < Fut : Future > Future for MaybeDone < Fut > { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { unsafe { match self . as_mut () . get_unchecked_mut () { Self :: Future (f) => { let res = ready ! (Pin :: new_unchecked (f) . poll (cx)) ; self . set (Self :: Done (res)) ; } Self :: Done (_) => { } Self :: Gone => panic ! ("MaybeDone polled after value taken") , } } Poll :: Ready (()) } }
    };
}

impl_164!()