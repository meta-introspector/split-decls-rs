// Generated macro for Utc (struct)
macro_rules! Depcrate_offset_utcUtc {
() => {
// Module: crate::offset::utc
// Provides: {"Utc"}
// Dependencies: {}
# [doc = " The UTC time zone. This is the most efficient time zone when you don't need the local time."] # [doc = " It is also used as an offset (which is also a dummy type)."] # [doc = ""] # [doc = " Using the [`TimeZone`](./trait.TimeZone.html) methods"] # [doc = " on the UTC struct is the preferred way to construct `DateTime<Utc>`"] # [doc = " instances."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{DateTime, TimeZone, Utc};"] # [doc = ""] # [doc = " let dt = DateTime::from_timestamp(61, 0).unwrap();"] # [doc = ""] # [doc = " assert_eq!(Utc.timestamp_opt(61, 0).unwrap(), dt);"] # [doc = " assert_eq!(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap(), dt);"] # [doc = " ```"] # [derive (Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (any (feature = "rkyv" , feature = "rkyv-16" , feature = "rkyv-32" , feature = "rkyv-64") , derive (Archive , Deserialize , Serialize) , archive (compare (PartialEq)) , archive_attr (derive (Clone , Copy , PartialEq , Eq , Debug , Hash)))] # [cfg_attr (feature = "rkyv-validation" , archive (check_bytes))] # [cfg_attr (all (feature = "arbitrary" , feature = "std") , derive (arbitrary :: Arbitrary))] pub struct Utc ;
};
}
