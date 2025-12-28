macro_rules! deps {
    () => {
        ReadCacheInternal!();
        Result!();
        ReadCacheOps!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < R : ReadCacheOps > ReadCacheInternal < R > { # [doc = " Ensures this range is contained in the len of the file"] fn range_in_bounds (& mut self , range : & Range < u64 >) -> Result < () , () > { if range . start <= range . end && range . end <= self . len () ? { Ok (()) } else { Err (()) } } # [doc = " The length of the underlying read, memoized"] fn len (& mut self) -> Result < u64 , () > { match self . len { Some (len) => Ok (len) , None => { let len = self . read . len () ? ; self . len = Some (len) ; Ok (len) } } } }
    };
}

impl_88!();