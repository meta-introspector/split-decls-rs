// Generated macro for impl_414 (impl)
macro_rules! Depcrate_serializeimpl_414 {
() => {
// Module: crate::serialize
// Provides: {"impl_414"}
// Dependencies: {}
impl OwnedData { # [doc = " # Safety"] # [doc = ""] # [doc = " Caller must be certain that `ptr` is allocated by `sqlite3_malloc64`."] pub unsafe fn from_raw_nonnull (ptr : NonNull < u8 > , sz : usize) -> Self { Self { ptr , sz } } fn into_raw (self) -> (* mut u8 , usize) { let raw = (self . ptr . as_ptr () , self . sz) ; std :: mem :: forget (self) ; raw } }
};
}
