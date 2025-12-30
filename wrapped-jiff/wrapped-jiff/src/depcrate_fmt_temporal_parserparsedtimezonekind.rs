// Generated macro for ParsedTimeZoneKind (enum)
macro_rules! Depcrate_fmt_temporal_parserParsedTimeZoneKind {
() => {
// Module: crate::fmt::temporal::parser
// Provides: {"ParsedTimeZoneKind"}
// Dependencies: {}
# [derive (Debug)] pub (super) enum ParsedTimeZoneKind < 'i > { Named (& 'i str) , Offset (ParsedOffset) , # [cfg (feature = "alloc")] Posix (crate :: tz :: posix :: PosixTimeZoneOwned) , }
};
}
