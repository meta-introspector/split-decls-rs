// Generated macro for impl_974 (impl)
macro_rules! Depcrate_tz_dbimpl_974 {
() => {
// Module: crate::tz::db
// Provides: {"impl_974"}
// Dependencies: {}
impl < 'd > TimeZoneNameIter < 'd > { # [doc = " Creates a time zone name iterator that never yields any elements."] fn empty () -> TimeZoneNameIter < 'd > { # [cfg (feature = "alloc")] { TimeZoneNameIter { it : alloc :: vec :: Vec :: new () . into_iter () } } # [cfg (not (feature = "alloc"))] { TimeZoneNameIter { it : core :: iter :: empty () } } } # [doc = " Creates a time zone name iterator that yields the elements from the"] # [doc = " iterator given. (They are collected into a `Vec`.)"] # [cfg (feature = "alloc")] fn from_iter (it : impl Iterator < Item = impl Into < alloc :: string :: String > > ,) -> TimeZoneNameIter < 'd > { let names : alloc :: vec :: Vec < TimeZoneName < 'd > > = it . map (| name | TimeZoneName :: new (name . into ())) . collect () ; TimeZoneNameIter { it : names . into_iter () } } }
};
}
