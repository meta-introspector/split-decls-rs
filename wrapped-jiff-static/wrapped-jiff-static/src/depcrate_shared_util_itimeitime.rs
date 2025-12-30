// Generated macro for ITime (struct)
macro_rules! Depcrate_shared_util_itimeITime {
() => {
// Module: crate::shared::util::itime
// Provides: {"ITime"}
// Dependencies: {}
# [doc = " Represents a clock time."] # [doc = ""] # [doc = " This uses units of hours, minutes, seconds and fractional seconds (to"] # [doc = " nanosecond precision)."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord)] pub (crate) struct ITime { pub (crate) hour : i8 , pub (crate) minute : i8 , pub (crate) second : i8 , pub (crate) subsec_nanosecond : i32 , }
};
}
