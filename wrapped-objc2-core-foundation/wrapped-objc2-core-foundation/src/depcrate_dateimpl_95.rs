// Generated macro for impl_95 (impl)
macro_rules! Depcrate_dateimpl_95 {
() => {
// Module: crate::date
// Provides: {"impl_95"}
// Dependencies: {}
impl CFDate { # [doc = " Create a `CFDate` from a [`SystemTime`]."] # [doc = ""] # [doc = " Nanosecond precision may be lost."] # [doc = ""] # [doc = " [`SystemTime`]: std::time::SystemTime"] # [cfg (feature = "std")] pub fn from_system_time (time : & std :: time :: SystemTime) -> crate :: CFRetained < Self > { let since_1970 = match time . duration_since (std :: time :: UNIX_EPOCH) { Ok (duration) => duration . as_secs_f64 () , Err (err) => - err . duration () . as_secs_f64 () , } as core :: ffi :: c_double ; let since_2001 = since_1970 - unsafe { crate :: kCFAbsoluteTimeIntervalSince1970 } ; Self :: new (None , since_2001) . expect ("failed creating CFDate") } # [doc = " Try to construct a [`SystemTime`] from the `CFDate`."] # [doc = ""] # [doc = " Nanosecond precision may be lost."] # [doc = ""] # [doc = " Returns `None` if the `CFDate` is too large to fit inside"] # [doc = " [`SystemTime`]."] # [doc = ""] # [doc = " [`SystemTime`]: std::time::SystemTime"] # [cfg (feature = "std")] # [allow (clippy :: unnecessary_cast)] pub fn to_system_time (& self) -> Option < std :: time :: SystemTime > { let since_2001 = self . absolute_time () ; let since_1970 = (since_2001 + unsafe { crate :: kCFAbsoluteTimeIntervalSince1970 }) as f64 ; std :: time :: UNIX_EPOCH . checked_add (std :: time :: Duration :: try_from_secs_f64 (since_1970) . ok () ?) } }
};
}
