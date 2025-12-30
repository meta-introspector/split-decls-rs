// Generated macro for impl_120 (impl)
macro_rules! Depcrate_read_read_cacheimpl_120 {
() => {
// Module: crate::read::read_cache
// Provides: {"impl_120"}
// Dependencies: {}
impl < R : ReadCacheOps > ReadCacheInternal < R > { # [doc = " Ensures this range is contained in the len of the file"] fn range_in_bounds (& mut self , range : & Range < u64 >) -> Result < () , () > { if range . start <= range . end && range . end <= self . len () ? { Ok (()) } else { Err (()) } } # [doc = " The length of the underlying read, memoized"] fn len (& mut self) -> Result < u64 , () > { match self . len { Some (len) => Ok (len) , None => { let len = self . read . len () ? ; self . len = Some (len) ; Ok (len) } } } }
};
}
