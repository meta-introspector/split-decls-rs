macro_rules! time {
    () => {
        # [cfg (any (feature = "jiff" , feature = "local-time"))] # [doc = ""] pub mod time ;
    };
}

time!()