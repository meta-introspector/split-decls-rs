macro_rules! WANT_READY {
    () => {
        # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] const WANT_READY : usize = 2 ;
    };
}

WANT_READY!()