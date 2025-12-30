// Generated macro for impl_390 (impl)
macro_rules! Depcrate_fmt_rfc9557impl_390 {
() => {
// Module: crate::fmt::rfc9557
// Provides: {"impl_390"}
// Dependencies: {}
impl < 'i > ParsedTimeZone < 'i > { # [doc = " Turns this parsed time zone into a structured time zone annotation."] # [doc = ""] # [doc = " This can return an error if the parsed offset could not be converted"] # [doc = " to a `crate::tz::Offset`."] # [doc = ""] # [doc = " This also includes a flag of whether the annotation is \"critical\" or"] # [doc = " not."] pub (crate) fn to_time_zone_annotation (& self ,) -> Result < TimeZoneAnnotation < 'i > , Error > { let (kind , critical) = match * self { ParsedTimeZone :: Named { name , critical } => { let kind = TimeZoneAnnotationKind :: from (name) ; (kind , critical) } ParsedTimeZone :: Offset { ref offset , critical } => { let kind = TimeZoneAnnotationKind :: Offset (offset . to_offset () ?) ; (kind , critical) } } ; Ok (TimeZoneAnnotation { kind , critical }) } }
};
}
