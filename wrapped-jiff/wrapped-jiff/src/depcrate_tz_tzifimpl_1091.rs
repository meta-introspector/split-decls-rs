// Generated macro for impl_1091 (impl)
macro_rules! Depcrate_tz_tzifimpl_1091 {
() => {
// Module: crate::tz::tzif
// Provides: {"impl_1091"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TzifOwned { # [doc = " Parses the given data as a TZif formatted file."] # [doc = ""] # [doc = " The name given is attached to the `Tzif` value returned, but is"] # [doc = " otherwise not significant."] # [doc = ""] # [doc = " If the given data is not recognized to be valid TZif, then an error is"] # [doc = " returned."] # [doc = ""] # [doc = " In general, callers may assume that it is safe to pass arbitrary or"] # [doc = " even untrusted data to this function and count on it not panicking"] # [doc = " or using resources that aren't limited to a small constant factor of"] # [doc = " the size of the data itself. That is, callers can reliably limit the"] # [doc = " resources used by limiting the size of the data given to this parse"] # [doc = " function."] pub (crate) fn parse (name : Option < String > , bytes : & [u8] ,) -> Result < Self , Error > { let sh = shared :: TzifOwned :: parse (name , bytes) . map_err (Error :: shared) ? ; Ok (TzifOwned :: from_shared_owned (sh)) } # [doc = " Converts from the shared-but-internal API for use in proc macros."] # [doc = ""] # [doc = " This is not `const` since it accepts owned values on the heap for"] # [doc = " variable length data inside `Tzif`."] pub (crate) fn from_shared_owned (sh : shared :: TzifOwned) -> TzifOwned { let posix_tz = match sh . fixed . posix_tz { None => None , Some (posix_tz) => Some (PosixTimeZone :: from_shared_owned (posix_tz)) , } ; Tzif { inner : sh , posix_tz } } }
};
}
