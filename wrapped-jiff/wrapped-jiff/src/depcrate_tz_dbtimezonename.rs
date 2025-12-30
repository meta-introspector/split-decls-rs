// Generated macro for TimeZoneName (struct)
macro_rules! Depcrate_tz_dbTimeZoneName {
() => {
// Module: crate::tz::db
// Provides: {"TimeZoneName"}
// Dependencies: {}
# [doc = " A name for a time zone yield by the [`TimeZoneNameIter`] iterator."] # [doc = ""] # [doc = " The iterator is created by [`TimeZoneDatabase::available`]."] # [doc = ""] # [doc = " The lifetime parameter corresponds to the lifetime of the"] # [doc = " `TimeZoneDatabase` from which this name was created."] # [derive (Clone , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub struct TimeZoneName < 'd > { # [doc = " The lifetime of the tzdb."] # [doc = ""] # [doc = " We don't currently use this, but it could be quite useful if we ever"] # [doc = " adopt a \"compile time\" tzdb like what `chrono-tz` has. Then we could"] # [doc = " return strings directly from the embedded data. Or perhaps a \"compile"] # [doc = " time\" TZif or some such."] lifetime : core :: marker :: PhantomData < & 'd str > , # [cfg (feature = "alloc")] name : alloc :: string :: String , # [cfg (not (feature = "alloc"))] name : core :: convert :: Infallible , }
};
}
