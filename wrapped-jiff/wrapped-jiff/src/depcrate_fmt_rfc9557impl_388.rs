// Generated macro for impl_388 (impl)
macro_rules! Depcrate_fmt_rfc9557impl_388 {
() => {
// Module: crate::fmt::rfc9557
// Provides: {"impl_388"}
// Dependencies: {}
impl < 'i > ParsedAnnotations < 'i > { # [doc = " Return an empty parsed annotations."] pub (crate) fn none () -> ParsedAnnotations < 'static > { ParsedAnnotations { input : escape :: Bytes (& []) , time_zone : None } } # [doc = " Turns this parsed time zone into a structured time zone annotation,"] # [doc = " if an annotation was found. Otherwise, returns `Ok(None)`."] # [doc = ""] # [doc = " This can return an error if the parsed offset could not be converted"] # [doc = " to a `crate::tz::Offset`."] pub (crate) fn to_time_zone_annotation (& self ,) -> Result < Option < TimeZoneAnnotation < 'i > > , Error > { let Some (ref parsed) = self . time_zone else { return Ok (None) } ; Ok (Some (parsed . to_time_zone_annotation () ?)) } }
};
}
