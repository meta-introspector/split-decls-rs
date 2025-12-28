macro_rules! deps {
    () => {
        MaybeDone!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < Fut : Future > Future for MaybeDone < Fut > { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let res = unsafe { match Pin :: as_mut (& mut self) . get_unchecked_mut () { MaybeDone :: Future (a) => ready ! (Pin :: new_unchecked (a) . poll (cx)) , MaybeDone :: Done (_) => return Poll :: Ready (()) , MaybeDone :: Gone => panic ! ("MaybeDone polled after value taken") , } } ; self . set (MaybeDone :: Done (res)) ; Poll :: Ready (()) } }
    };
}

impl_41!();