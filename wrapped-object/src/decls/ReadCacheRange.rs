macro_rules! deps {
    () => {
        ReadCache!();
        ReadCacheOps!();
        ReadRef!();
    };
}

macro_rules! ReadCacheRange {
    () => {
        deps!();
        # [doc = " An implementation of [`ReadRef`] for a range of data in a stream that"] # [doc = " implements `Read + Seek`."] # [doc = ""] # [doc = " Shares an underlying [`ReadCache`] with a lifetime of `'a`."] # [derive (Debug)] pub struct ReadCacheRange < 'a , R : ReadCacheOps > { r : & 'a ReadCache < R > , offset : u64 , size : u64 , }
    };
}

ReadCacheRange!();