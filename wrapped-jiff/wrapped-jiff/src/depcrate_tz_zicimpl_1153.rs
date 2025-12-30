// Generated macro for impl_1153 (impl)
macro_rules! Depcrate_tz_zicimpl_1153 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1153"}
// Dependencies: {}
impl FromStr for ZoneFormatP { type Err = Error ; fn from_str (format : & str) -> Result < ZoneFormatP , Error > { fn check_abbrev (abbrev : & str) -> Result < String , Error > { if abbrev . is_empty () { return Err (err ! ("empty abbreviations are not allowed")) ; } let is_ok = | ch | matches ! (ch , '+' |'-' |'0' ..='9' |'A' ..='Z' |'a' ..='z') ; if ! abbrev . chars () . all (is_ok) { return Err (err ! ("abbreviation {abbrev:?} \
                     contains invalid character; only \"+\", \"-\" and \
                     ASCII alpha-numeric characters are allowed")) ; } Ok (abbrev . to_string ()) } if format == "%z" { Ok (ZoneFormatP :: Offset) } else if let Some ((before , after)) = format . split_once ("%s") { Ok (ZoneFormatP :: Variable { before : check_abbrev (before) ? , after : check_abbrev (after) ? , }) } else if let Some ((std , dst)) = format . split_once ("/") { Ok (ZoneFormatP :: Pair { std : check_abbrev (std) ? , dst : check_abbrev (dst) ? , }) } else { Ok (ZoneFormatP :: Static { format : check_abbrev (format) ? }) } } }
};
}
