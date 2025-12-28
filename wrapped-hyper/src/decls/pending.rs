macro_rules! deps {
    () => {
        Pending!();
        OnUpgrade!();
    };
}

macro_rules! pending {
    () => {
        deps!();
        # [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2") ,))] pub (super) fn pending () -> (Pending , OnUpgrade) { let (tx , rx) = oneshot :: channel () ; (Pending { tx } , OnUpgrade { rx : Some (Arc :: new (Mutex :: new (rx))) , } ,) }
    };
}

pending!();