// Generated macro for TimeZoneAnnotationName (struct)
macro_rules! Depcrate_fmt_temporal_piecesTimeZoneAnnotationName {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"TimeZoneAnnotationName"}
// Dependencies: {}
# [doc = " A time zone annotation parsed from a datetime string."] # [doc = ""] # [doc = " By default, a time zone annotation name borrows its name from the"] # [doc = " input it was parsed from. When the `alloc` feature is enabled,"] # [doc = " callers can de-couple the annotation from the parsed input with"] # [doc = " [`TimeZoneAnnotationName::into_owned`]."] # [doc = ""] # [doc = " A value of this type is usually found via [`Pieces::time_zone_annotation`],"] # [doc = " but callers can also construct one via this type's `From<&str>` trait"] # [doc = " implementation if necessary."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct TimeZoneAnnotationName < 'n > { name : StringCow < 'n > , }
};
}
