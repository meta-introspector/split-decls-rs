// Generated macro for TimeZoneAnnotationKind (enum)
macro_rules! Depcrate_fmt_temporal_piecesTimeZoneAnnotationKind {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"TimeZoneAnnotationKind"}
// Dependencies: {}
# [doc = " The kind of time zone found in an [RFC 9557] timestamp, for use with"] # [doc = " [`Pieces`]."] # [doc = ""] # [doc = " The lifetime parameter refers to the lifetime of the time zone"] # [doc = " name. The lifetime is static when the time zone annotation is"] # [doc = " offset or if the name is owned. An owned value can be produced via"] # [doc = " [`TimeZoneAnnotation::into_owned`] when the `alloc` crate feature is"] # [doc = " enabled."] # [doc = ""] # [doc = " [RFC 9557]: https://www.rfc-editor.org/rfc/rfc9557.html"] # [derive (Clone , Debug , Eq , Hash , PartialEq)] # [non_exhaustive] pub enum TimeZoneAnnotationKind < 'n > { # [doc = " The time zone annotation is a name, usually an IANA time zone"] # [doc = " identifier. For example, `America/New_York`."] Named (TimeZoneAnnotationName < 'n >) , # [doc = " The time zone annotation is an offset. For example, `-05:00`."] Offset (Offset) , }
};
}
