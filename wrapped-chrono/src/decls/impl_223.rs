macro_rules! deps {
    () => {
        ParseError!();
        ParseResult!();
        FixedOffset!();
        Parsed!();
        DateTime!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        # [doc = " Accepts a relaxed form of RFC3339."] # [doc = " A space or a 'T' are accepted as the separator between the date and time"] # [doc = " parts. Additional spaces are allowed between each component."] # [doc = ""] # [doc = " All of these examples are equivalent:"] # [doc = " ```"] # [doc = " # use chrono::{DateTime, offset::FixedOffset};"] # [doc = " \"2012-12-12T12:12:12Z\".parse::<DateTime<FixedOffset>>()?;"] # [doc = " \"2012-12-12 12:12:12Z\".parse::<DateTime<FixedOffset>>()?;"] # [doc = " \"2012-  12-12T12:  12:12Z\".parse::<DateTime<FixedOffset>>()?;"] # [doc = " # Ok::<(), chrono::ParseError>(())"] # [doc = " ```"] impl str :: FromStr for DateTime < FixedOffset > { type Err = ParseError ; fn from_str (s : & str) -> ParseResult < DateTime < FixedOffset > > { let mut parsed = Parsed :: new () ; let (s , _) = parse_rfc3339_relaxed (& mut parsed , s) ? ; if ! s . trim_start () . is_empty () { return Err (TOO_LONG) ; } parsed . to_datetime () } }
    };
}

impl_223!()