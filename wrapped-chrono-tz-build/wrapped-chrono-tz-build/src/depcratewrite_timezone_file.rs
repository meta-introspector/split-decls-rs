// Generated macro for write_timezone_file (function)
macro_rules! Depcratewrite_timezone_file {
() => {
// Module: crate
// Provides: {"write_timezone_file"}
// Dependencies: {}
fn write_timezone_file (timezone_file : & mut File , table : & Table , uncased : bool) -> io :: Result < () > { let zones = table . zonesets . keys () . chain (table . links . keys ()) . collect :: < BTreeSet < _ > > () ; writeln ! (timezone_file , "use core::fmt::{{self, Debug, Display, Formatter}};" ,) ? ; writeln ! (timezone_file , "use core::str::FromStr;\n" ,) ? ; writeln ! (timezone_file , "use crate::timezone_impl::{{TimeSpans, FixedTimespanSet, FixedTimespan}};\n" ,) ? ; writeln ! (timezone_file , "/// TimeZones built at compile time from the tz database
///
/// This implements [`chrono::TimeZone`] so that it may be used in and to
/// construct chrono's DateTime type. See the root module documentation
/// for details.") ? ; writeln ! (timezone_file , "#[derive(Clone, Copy, PartialEq, Eq, Hash)]") ? ; writeln ! (timezone_file , r#"#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]"#) ? ; writeln ! (timezone_file , "pub enum Tz {{") ? ; for zone in & zones { let zone_name = convert_bad_chars (zone) ; writeln ! (timezone_file , "    /// {zone}\n    {zone_name},") ? ; } writeln ! (timezone_file , "}}") ? ; let mut map = phf_codegen :: Map :: new () ; for zone in & zones { map . entry (zone , format ! ("Tz::{}" , convert_bad_chars (zone))) ; } writeln ! (timezone_file , "static TIMEZONES: ::phf::Map<&'static str, Tz> = \n{};" , map . build ()) ? ; # [cfg (feature = "case-insensitive")] if uncased { writeln ! (timezone_file , "use uncased::UncasedStr;\n" ,) ? ; let mut map = phf_codegen :: Map :: new () ; for zone in & zones { map . entry (uncased :: UncasedStr :: new (zone) , format ! ("Tz::{}" , convert_bad_chars (zone)) ,) ; } writeln ! (timezone_file , "static TIMEZONES_UNCASED: ::phf::Map<&'static uncased::UncasedStr, Tz> = \n{};" , map . build ()) ? ; } writeln ! (timezone_file , "#[cfg_attr(feature = \"defmt\", derive(defmt::Format))]") ? ; writeln ! (timezone_file , r#"#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ParseError(());

impl Display for ParseError {{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {{
        f.write_str("failed to parse timezone")
    }}
}}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {{}}

impl FromStr for Tz {{
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        TIMEZONES.get(s).cloned().ok_or(ParseError(()))
    }}
}}
"#) ? ; writeln ! (timezone_file , "impl Tz {{
    pub fn name(self) -> &'static str {{
        match self {{") ? ; for zone in & zones { let zone_name = convert_bad_chars (zone) ; writeln ! (timezone_file , "            Tz::{zone_name} => \"{zone}\",") ? ; } writeln ! (timezone_file , "        }}
    }}") ? ; if uncased { writeln ! (timezone_file , r#"
    #[cfg(feature = "case-insensitive")]
    /// Parses a timezone string in a case-insensitive way
    pub fn from_str_insensitive(s: &str) -> Result<Self, ParseError> {{
        return TIMEZONES_UNCASED.get(s.into()).cloned().ok_or(ParseError(()));
    }}"#) ? ; } writeln ! (timezone_file , "}}") ? ; writeln ! (timezone_file , "impl Debug for Tz {{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {{
        f.write_str(self.name().as_ref())
    }}
}}\n") ? ; writeln ! (timezone_file , "impl Display for Tz {{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {{
        f.write_str(self.name().as_ref())
    }}
}}\n") ? ; writeln ! (timezone_file , r#"#[cfg(feature = "defmt")]
impl defmt::Format for Tz {{
    fn format(&self, f: defmt::Formatter) {{
        defmt::write!(f, "{{:?}}", self.name());
    }}
}}
"#) ? ; writeln ! (timezone_file , "impl TimeSpans for Tz {{
    fn timespans(&self) -> FixedTimespanSet {{") ? ; for zone in & zones { if table . links . contains_key (zone . as_str ()) { continue ; } let zone_name = convert_bad_chars (zone) ; let timespans = table . timespans (zone) . unwrap () ; writeln ! (timezone_file , "        const {zone}: FixedTimespanSet = FixedTimespanSet {{
            first: FixedTimespan {{ offset: {offset}, name: {name:?} }},
            rest: {rest},
        }};\n" , zone = zone_name . to_uppercase () , rest = format_rest (timespans . rest) , offset = timespans . first . utc_offset + timespans . first . dst_offset , name = timespans . first . name ,) ? ; } write ! (timezone_file , "
        match *self {{
") ? ; for zone in & zones { let zone_name = convert_bad_chars (zone) ; let target_name = if let Some (target) = table . links . get (zone . as_str ()) { convert_bad_chars (target) } else { zone_name . clone () } ; writeln ! (timezone_file , "            Tz::{zone_name} => {target_name}," , target_name = target_name . to_uppercase () ,) ? ; } write ! (timezone_file , "        }}
    }}
}}\n") ? ; write ! (timezone_file , "/// An array of every known variant
///
/// Useful for iterating over known timezones:
///
/// ```
/// use chrono_tz::{{TZ_VARIANTS, Tz}};
/// assert!(TZ_VARIANTS.iter().any(|v| *v == Tz::UTC));
/// ```
pub static TZ_VARIANTS: [Tz; {num}] = [
" , num = zones . len ()) ? ; for zone in & zones { writeln ! (timezone_file , "    Tz::{zone}," , zone = convert_bad_chars (zone)) ? ; } write ! (timezone_file , "];") ? ; Ok (()) }
};
}
