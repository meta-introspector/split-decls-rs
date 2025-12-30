// Generated macro for TimeZoneNameIter (struct)
macro_rules! Depcrate_tz_dbTimeZoneNameIter {
() => {
// Module: crate::tz::db
// Provides: {"TimeZoneNameIter"}
// Dependencies: {}
# [doc = " An iterator over the time zone identifiers in a [`TimeZoneDatabase`]."] # [doc = ""] # [doc = " This iterator is created by [`TimeZoneDatabase::available`]."] # [doc = ""] # [doc = " There are no guarantees about the order in which this iterator yields"] # [doc = " time zone identifiers."] # [doc = ""] # [doc = " The lifetime parameter corresponds to the lifetime of the"] # [doc = " `TimeZoneDatabase` from which this iterator was created."] # [derive (Clone , Debug)] pub struct TimeZoneNameIter < 'd > { # [cfg (feature = "alloc")] it : alloc :: vec :: IntoIter < TimeZoneName < 'd > > , # [cfg (not (feature = "alloc"))] it : core :: iter :: Empty < TimeZoneName < 'd > > , }
};
}
