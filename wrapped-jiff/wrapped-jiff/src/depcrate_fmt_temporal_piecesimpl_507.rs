// Generated macro for impl_507 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_507 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_507"}
// Dependencies: {}
impl < 'n > TimeZoneAnnotationName < 'n > { # [doc = " Returns the name of this time zone annotation as a string slice."] # [doc = ""] # [doc = " Note that the lifetime of the string slice returned is tied to the"] # [doc = " lifetime of this time zone annotation. This may be shorter than the"] # [doc = " lifetime of the string, `'n`, in this annotation."] # [inline] pub fn as_str < 'a > (& 'a self) -> & 'a str { self . name . as_str () } # [doc = " Converts this time zone annotation name into an \"owned\" value whose"] # [doc = " lifetime is `'static`."] # [doc = ""] # [doc = " If this was already an \"owned\" value, then this is a no-op."] # [cfg (feature = "alloc")] # [inline] pub fn into_owned (self) -> TimeZoneAnnotationName < 'static > { TimeZoneAnnotationName { name : self . name . into_owned () } } }
};
}
