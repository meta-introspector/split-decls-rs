// Generated macro for Year (type)
macro_rules! Depcrate_util_tYear {
() => {
// Module: crate::util::t
// Provides: {"Year"}
// Dependencies: {}
# [doc = " The range of years supported by jiff."] # [doc = ""] # [doc = " This is ultimately where some of the other ranges (like `UnixSeconds`)"] # [doc = " were determined from. That is, the range of years is the primary point at"] # [doc = " which the space of supported time instants is derived from. If one wanted"] # [doc = " to expand this range, you'd need to change it here and then compute the"] # [doc = " corresponding min/max values for `UnixSeconds`."] pub (crate) type Year = ri16 < - 9999 , 9999 > ;
};
}
