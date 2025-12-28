macro_rules! cfg_proto {
    () => {
        macro_rules ! cfg_proto { ($ ($ item : item) *) => { cfg_feature ! { #! [all (any (feature = "http1" , feature = "http2") , any (feature = "client" , feature = "server") ,)] $ ($ item) * } } }
    };
}

cfg_proto!();