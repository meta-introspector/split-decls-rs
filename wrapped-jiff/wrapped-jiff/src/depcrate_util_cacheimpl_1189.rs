// Generated macro for impl_1189 (impl)
macro_rules! Depcrate_util_cacheimpl_1189 {
() => {
// Module: crate::util::cache
// Provides: {"impl_1189"}
// Dependencies: {}
impl Expiration { # [doc = " Returns an expiration time for which `is_expired` returns true after"] # [doc = " the given duration has elapsed from this instant."] pub (crate) fn after (ttl : Duration) -> Expiration { Expiration (crate :: now :: monotonic_time () . and_then (| now | now . checked_add (ttl)) ,) } # [doc = " Returns an expiration time for which `is_expired` always returns true."] pub (crate) const fn expired () -> Expiration { Expiration (None) } # [doc = " Whether expiration has occurred or not."] pub (crate) fn is_expired (self) -> bool { self . 0 . map_or (true , | t | { let Some (now) = crate :: now :: monotonic_time () else { return true } ; now > t }) } }
};
}
