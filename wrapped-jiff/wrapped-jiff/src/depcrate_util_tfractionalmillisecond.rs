// Generated macro for FractionalMillisecond (type)
macro_rules! Depcrate_util_tFractionalMillisecond {
() => {
// Module: crate::util::t
// Provides: {"FractionalMillisecond"}
// Dependencies: {}
# [doc = " The range of allowable fractional milliseconds."] # [doc = ""] # [doc = " That is, this corresponds to the range of milliseconds allowable within a"] # [doc = " single second. It can be either positive or negative."] pub (crate) type FractionalMillisecond = ri32 < { - (MILLIS_PER_SECOND . bound () - 1) } , { MILLIS_PER_SECOND . bound () - 1 } , > ;
};
}
