// Generated macro for FractionalNanosecond (type)
macro_rules! Depcrate_util_tFractionalNanosecond {
() => {
// Module: crate::util::t
// Provides: {"FractionalNanosecond"}
// Dependencies: {}
# [doc = " The range of allowable fractional nanoseconds."] # [doc = ""] # [doc = " That is, this corresponds to the range of nanoseconds allowable within a"] # [doc = " single second. It can be either positive or negative."] pub (crate) type FractionalNanosecond = ri32 < { - (NANOS_PER_SECOND . bound () - 1) } , { NANOS_PER_SECOND . bound () - 1 } , > ;
};
}
