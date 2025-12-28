macro_rules! buf {
    () => {
        # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] pub (crate) mod buf ;
    };
}

buf!();