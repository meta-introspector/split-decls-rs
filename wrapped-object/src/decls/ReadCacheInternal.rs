macro_rules! deps {
    () => {
        ReadCacheOps!();
    };
}

macro_rules! ReadCacheInternal {
    () => {
        deps!();
        # [derive (Debug)] struct ReadCacheInternal < R : ReadCacheOps > { read : R , bufs : Map < (u64 , u64) , Box < [u8] > > , strings : Map < (u64 , u8) , Box < [u8] > > , len : Option < u64 > , }
    };
}

ReadCacheInternal!();