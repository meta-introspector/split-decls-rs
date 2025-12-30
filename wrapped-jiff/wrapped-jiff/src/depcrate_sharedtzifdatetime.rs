// Generated macro for TzifDateTime (struct)
macro_rules! Depcrate_sharedTzifDateTime {
() => {
// Module: crate::shared
// Provides: {"TzifDateTime"}
// Dependencies: {}
# [doc = " The representation we use to represent a civil datetime."] # [doc = ""] # [doc = " We don't use `shared::util::itime::IDateTime` here because we specifically"] # [doc = " do not need to represent fractional seconds. This lets us easily represent"] # [doc = " what we need in 8 bytes instead of the 12 bytes used by `IDateTime`."] # [doc = ""] # [doc = " Moreover, we pack the fields into a single `i64` to make comparisons"] # [doc = " extremely cheap. This is especially useful since we do a binary search on"] # [doc = " `&[TzifDateTime]` when doing a TZ lookup for a civil datetime."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub struct TzifDateTime { bits : i64 , }
};
}
