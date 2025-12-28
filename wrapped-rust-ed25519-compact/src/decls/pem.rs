macro_rules! pem {
    () => {
        # [cfg (not (feature = "disable-signatures"))] # [cfg (feature = "pem")] mod pem ;
    };
}

pem!();