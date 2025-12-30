// Generated macro for TzifOwned (type)
macro_rules! Depcrate_tz_tzifTzifOwned {
() => {
// Module: crate::tz::tzif
// Provides: {"TzifOwned"}
// Dependencies: {}
# [doc = " The owned variant of `Tzif`."] # [cfg (feature = "alloc")] pub (crate) type TzifOwned = Tzif < String , Abbreviation , Vec < shared :: TzifLocalTimeType > , Vec < i64 > , Vec < shared :: TzifDateTime > , Vec < shared :: TzifDateTime > , Vec < shared :: TzifTransitionInfo > , > ;
};
}
