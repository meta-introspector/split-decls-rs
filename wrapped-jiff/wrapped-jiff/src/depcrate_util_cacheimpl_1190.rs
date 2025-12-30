// Generated macro for impl_1190 (impl)
macro_rules! Depcrate_util_cacheimpl_1190 {
() => {
// Module: crate::util::cache
// Provides: {"impl_1190"}
// Dependencies: {}
impl core :: fmt :: Display for Expiration { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let Some (instant) = self . 0 else { return write ! (f , "expired") ; } ; let Some (now) = crate :: now :: monotonic_time () else { return write ! (f , "expired") ; } ; let Some (duration) = instant . checked_duration_since (now) else { return write ! (f , "expired") ; } ; write ! (f , "{duration:?}") } }
};
}
