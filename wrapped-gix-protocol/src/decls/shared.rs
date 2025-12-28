macro_rules! shared {
    () => {
        # [cfg (any (feature = "blocking-client" , feature = "async-client"))] pub (crate) mod shared ;
    };
}

shared!()