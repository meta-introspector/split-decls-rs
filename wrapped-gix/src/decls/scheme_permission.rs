macro_rules! scheme_permission {
    () => {
        # [doc = ""] # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client"))] pub mod scheme_permission ;
    };
}

scheme_permission!()