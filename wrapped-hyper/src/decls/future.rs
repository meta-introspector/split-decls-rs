macro_rules! future {
    () => {
        # [cfg (any (all (feature = "client" , any (feature = "http1" , feature = "http2")) , all (feature = "server" , feature = "http1") ,))] pub (crate) mod future ;
    };
}

future!()