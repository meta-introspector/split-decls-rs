macro_rules! redirect {
    () => {
        # [doc = ""] # [cfg (any (feature = "http-client-curl" , feature = "http-client-reqwest"))] pub mod redirect ;
    };
}

redirect!()