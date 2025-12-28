macro_rules! connect {
    () => {
        # [doc = ""] # [cfg (any (feature = "async-network-client" , feature = "blocking-network-client"))] pub mod connect ;
    };
}

connect!();