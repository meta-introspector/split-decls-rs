macro_rules! WANT_PENDING {
    () => {
        # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] const WANT_PENDING : usize = 1 ;
    };
}

WANT_PENDING!();