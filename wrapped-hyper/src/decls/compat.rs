macro_rules! compat {
    () => {
        # [cfg (all (any (feature = "client" , feature = "server") , feature = "http2"))] mod compat ;
    };
}

compat!()