// Generated macro for impl_475 (impl)
macro_rules! Depcrate_fmt_temporal_parserimpl_475 {
() => {
// Module: crate::fmt::temporal::parser
// Provides: {"impl_475"}
// Dependencies: {}
impl < 'i > ParsedTimeZone < 'i > { pub (super) fn into_time_zone (self , db : & TimeZoneDatabase ,) -> Result < TimeZone , Error > { match self . kind { ParsedTimeZoneKind :: Named (iana_name) => { let tz = db . get (iana_name) . with_context (| | { err ! ("parsed apparent IANA time zone identifier \
                         {iana_name} from {input}, but the tzdb lookup \
                         failed" , input = self . input ,) }) ? ; Ok (tz) } ParsedTimeZoneKind :: Offset (poff) => { let offset = poff . to_offset () . with_context (| | { err ! ("offset successfully parsed from {input}, \
                         but failed to convert to numeric `Offset`" , input = self . input ,) }) ? ; Ok (TimeZone :: fixed (offset)) } # [cfg (feature = "alloc")] ParsedTimeZoneKind :: Posix (posix_tz) => { Ok (TimeZone :: from_posix_tz (posix_tz)) } } } }
};
}
