macro_rules! deps {
    () => {
        ReadCacheOps!();
        ReadCacheInternal!();
        ReadCache!();
        ReadRef!();
        ReadCacheRange!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < R : ReadCacheOps > ReadCache < R > { # [doc = " Create an empty `ReadCache` for the given stream."] pub fn new (read : R) -> Self { ReadCache { cache : RefCell :: new (ReadCacheInternal { read , bufs : Map :: new () , strings : Map :: new () , len : None , }) , } } # [doc = " Return an implementation of `ReadRef` that restricts reads"] # [doc = " to the given range of the stream."] pub fn range (& self , offset : u64 , size : u64) -> ReadCacheRange < '_ , R > { ReadCacheRange { r : self , offset , size , } } # [doc = " Free buffers used by the cache."] pub fn clear (& mut self) { self . cache . borrow_mut () . bufs . clear () ; } # [doc = " Unwrap this `ReadCache<R>`, returning the underlying reader."] pub fn into_inner (self) -> R { self . cache . into_inner () . read } }
    };
}

impl_89!()