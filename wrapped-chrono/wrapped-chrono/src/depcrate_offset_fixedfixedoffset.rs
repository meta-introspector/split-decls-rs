// Generated macro for FixedOffset (struct)
macro_rules! Depcrate_offset_fixedFixedOffset {
() => {
// Module: crate::offset::fixed
// Provides: {"FixedOffset"}
// Dependencies: {}
# [doc = " The time zone with fixed offset, from UTC-23:59:59 to UTC+23:59:59."] # [doc = ""] # [doc = " Using the [`TimeZone`](./trait.TimeZone.html) methods"] # [doc = " on a `FixedOffset` struct is the preferred way to construct"] # [doc = " `DateTime<FixedOffset>` instances. See the [`east_opt`](#method.east_opt) and"] # [doc = " [`west_opt`](#method.west_opt) methods for examples."] # [derive (PartialEq , Eq , Hash , Copy , Clone)] # [cfg_attr (any (feature = "rkyv" , feature = "rkyv-16" , feature = "rkyv-32" , feature = "rkyv-64") , derive (Archive , Deserialize , Serialize) , archive (compare (PartialEq)) , archive_attr (derive (Clone , Copy , PartialEq , Eq , Hash , Debug)))] # [cfg_attr (feature = "rkyv-validation" , archive (check_bytes))] pub struct FixedOffset { local_minus_utc : i32 , }
};
}
