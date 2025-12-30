// Generated macro for impl_489 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_489 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_489"}
// Dependencies: {}
impl < 'a > From < & 'a Zoned > for Pieces < 'a > { # [inline] fn from (zdt : & 'a Zoned) -> Pieces < 'a > { let mut pieces = Pieces :: from (zdt . datetime ()) . with_offset (zdt . offset ()) ; if let Some (name) = zdt . time_zone () . iana_name () { pieces = pieces . with_time_zone_name (name) ; } else { pieces = pieces . with_time_zone_offset (zdt . offset ()) ; } pieces } }
};
}
