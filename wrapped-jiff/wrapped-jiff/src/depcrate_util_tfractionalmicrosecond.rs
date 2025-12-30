// Generated macro for FractionalMicrosecond (type)
macro_rules! Depcrate_util_tFractionalMicrosecond {
() => {
// Module: crate::util::t
// Provides: {"FractionalMicrosecond"}
// Dependencies: {}
# [doc = " The range of allowable fractional microseconds."] # [doc = ""] # [doc = " That is, this corresponds to the range of microseconds allowable within a"] # [doc = " single second. It can be either positive or negative."] pub (crate) type FractionalMicrosecond = ri32 < { - (MICROS_PER_SECOND . bound () - 1) } , { MICROS_PER_SECOND . bound () - 1 } , > ;
};
}
