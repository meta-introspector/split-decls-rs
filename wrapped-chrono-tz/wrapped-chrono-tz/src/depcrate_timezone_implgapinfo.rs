// Generated macro for GapInfo (struct)
macro_rules! Depcrate_timezone_implGapInfo {
() => {
// Module: crate::timezone_impl
// Provides: {"GapInfo"}
// Dependencies: {}
# [doc = " Represents the information of a gap."] # [doc = ""] # [doc = " This returns useful information that can be used when converting a local [`NaiveDateTime`]"] # [doc = " to a timezone-aware [`DateTime`] with [`TimeZone::from_local_datetime`] and a gap"] # [doc = " ([`LocalResult::None`]) is found."] pub struct GapInfo { # [doc = " When available it contains information about the beginning of the gap."] # [doc = ""] # [doc = " The time represents the first instant in which the gap starts."] # [doc = " This means that it is the first instant that when used with [`TimeZone::from_local_datetime`]"] # [doc = " it will return [`LocalResult::None`]."] # [doc = ""] # [doc = " The offset represents the offset of the first instant before the gap."] pub begin : Option < (NaiveDateTime , TzOffset) > , # [doc = " When available it contains the first instant after the gap."] pub end : Option < DateTime < Tz > > , }
};
}
