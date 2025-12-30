// Generated macro for Expiration (struct)
macro_rules! Depcrate_util_cacheExpiration {
() => {
// Module: crate::util::cache
// Provides: {"Expiration"}
// Dependencies: {}
# [doc = " A little helper for representing expiration time."] # [doc = ""] # [doc = " An overflowing expiration time is treated identically to a time that is"] # [doc = " always expired."] # [doc = ""] # [doc = " When `None` internally, it implies that the expiration time is at some"] # [doc = " arbitrary point in the past beyond all possible \"time to live\" values."] # [doc = " i.e., A `None` value invalidates the cache at the next failed lookup."] # [derive (Clone , Copy , Debug)] pub (crate) struct Expiration (Option < MonotonicInstant >) ;
};
}
