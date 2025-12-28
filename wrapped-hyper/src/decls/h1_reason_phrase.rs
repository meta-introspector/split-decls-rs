macro_rules! h1_reason_phrase {
    () => {
        # [cfg (any (feature = "http1" , feature = "ffi"))] mod h1_reason_phrase ;
    };
}

h1_reason_phrase!();