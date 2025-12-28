macro_rules! macro_82 {
    () => {
        # [cfg (all (feature = "blocking-client" , feature = "async-client"))] compile_error ! ("Cannot set both 'blocking-client' and 'async-client' features as they are mutually exclusive") ;
    };
}

macro_82!()