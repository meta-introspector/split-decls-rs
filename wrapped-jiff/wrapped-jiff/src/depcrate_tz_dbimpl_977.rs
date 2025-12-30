// Generated macro for impl_977 (impl)
macro_rules! Depcrate_tz_dbimpl_977 {
() => {
// Module: crate::tz::db
// Provides: {"impl_977"}
// Dependencies: {}
impl < 'd > TimeZoneName < 'd > { # [doc = " Returns a new time zone name from the string given."] # [doc = ""] # [doc = " The lifetime returned is inferred according to the caller's context."] # [cfg (feature = "alloc")] fn new (name : alloc :: string :: String) -> TimeZoneName < 'd > { TimeZoneName { lifetime : core :: marker :: PhantomData , name } } # [doc = " Returns this time zone name as a borrowed string."] # [doc = ""] # [doc = " Note that the lifetime of the string returned is tied to `self`,"] # [doc = " which may be shorter than the lifetime `'d` of the originating"] # [doc = " `TimeZoneDatabase`."] # [inline] pub fn as_str < 'a > (& 'a self) -> & 'a str { # [cfg (feature = "alloc")] { self . name . as_str () } # [cfg (not (feature = "alloc"))] { unreachable ! () } } }
};
}
