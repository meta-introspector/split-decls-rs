macro_rules! MAX_LEN {
    () => {
        # [cfg (any (feature = "http1" , feature = "http2" , test))] const MAX_LEN : u64 = u64 :: MAX - 2 ;
    };
}

MAX_LEN!()