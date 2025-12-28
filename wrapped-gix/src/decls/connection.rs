macro_rules! connection {
    () => {
        # [cfg (any (feature = "async-network-client" , feature = "blocking-network-client"))] mod connection ;
    };
}

connection!();