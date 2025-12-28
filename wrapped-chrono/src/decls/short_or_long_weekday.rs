macro_rules! deps {
    () => {
        Weekday!();
        ParseResult!();
    };
}

macro_rules! short_or_long_weekday {
    () => {
        deps!();
        # [doc = " Tries to parse the weekday with short or long weekday names."] # [doc = " It prefers long weekday names to short weekday names when both are possible."] pub (super) fn short_or_long_weekday (s : & str) -> ParseResult < (& str , Weekday) > { static LONG_WEEKDAY_SUFFIXES : [& [u8] ; 7] = [b"day" , b"sday" , b"nesday" , b"rsday" , b"day" , b"urday" , b"day"] ; let (mut s , weekday) = short_weekday (s) ? ; let suffix = LONG_WEEKDAY_SUFFIXES [weekday . num_days_from_monday () as usize] ; if s . len () >= suffix . len () && s . as_bytes () [.. suffix . len ()] . eq_ignore_ascii_case (suffix) { s = & s [suffix . len () ..] ; } Ok ((s , weekday)) }
    };
}

short_or_long_weekday!()