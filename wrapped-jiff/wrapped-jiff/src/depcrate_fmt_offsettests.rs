// Generated macro for tests (module)
macro_rules! Depcrate_fmt_offsettests {
() => {
// Module: crate::fmt::offset
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: util :: rangeint :: RInto ; use super :: * ; # [test] fn ok_zulu () { let p = | input | Parser :: new () . parse (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"Z") , @ r###"
        Parsed {
            value: ParsedOffset {
                kind: Zulu,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"z") , @ r###"
        Parsed {
            value: ParsedOffset {
                kind: Zulu,
            },
            input: "",
        }
        "###) ; } # [test] fn ok_numeric () { let p = | input | Parser :: new () . parse (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"-05") , @ r###"
        Parsed {
            value: ParsedOffset {
                kind: Numeric(
                    -05,
                ),
            },
            input: "",
        }
        "###) ; } # [test] fn ok_numeric_complete () { let p = | input | Parser :: new () . parse_numeric (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"-05") , @ r###"
        Parsed {
            value: -05,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+05") , @ r###"
        Parsed {
            value: +05,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+25:59") , @ r###"
        Parsed {
            value: +25:59,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+2559") , @ r###"
        Parsed {
            value: +25:59,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+25:59:59") , @ r###"
        Parsed {
            value: +25:59:59,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+255959") , @ r###"
        Parsed {
            value: +25:59:59,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+25:59:59.999") , @ r###"
        Parsed {
            value: +25:59:59.999,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+25:59:59,999") , @ r###"
        Parsed {
            value: +25:59:59.999,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+255959.999") , @ r###"
        Parsed {
            value: +25:59:59.999,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+255959,999") , @ r###"
        Parsed {
            value: +25:59:59.999,
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"+25:59:59.999999999") , @ r###"
        Parsed {
            value: +25:59:59.999999999,
            input: "",
        }
        "###) ; } # [test] fn ok_numeric_incomplete () { let p = | input | Parser :: new () . parse_numeric (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"-05a") , @ r###"
        Parsed {
            value: -05,
            input: "a",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-05:12a") , @ r###"
        Parsed {
            value: -05:12,
            input: "a",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-05:12.") , @ r###"
        Parsed {
            value: -05:12,
            input: ".",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-05:12,") , @ r###"
        Parsed {
            value: -05:12,
            input: ",",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-0512a") , @ r###"
        Parsed {
            value: -05:12,
            input: "a",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-0512:") , @ r###"
        Parsed {
            value: -05:12,
            input: ":",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-05:12:34a") , @ r###"
        Parsed {
            value: -05:12:34,
            input: "a",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-05:12:34.9a") , @ r###"
        Parsed {
            value: -05:12:34.9,
            input: "a",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-05:12:34.9.") , @ r###"
        Parsed {
            value: -05:12:34.9,
            input: ".",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"-05:12:34.9,") , @ r###"
        Parsed {
            value: -05:12:34.9,
            input: ",",
        }
        "###) ; } # [test] fn err_numeric_empty () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"") . unwrap_err () , @ r###"failed to parse sign in UTC numeric offset "": expected UTC numeric offset, but found end of input"### ,) ; } # [test] fn err_numeric_notsign () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"*") . unwrap_err () , @ r###"failed to parse sign in UTC numeric offset "*": expected '+' or '-' sign at start of UTC numeric offset, but found "*" instead"### ,) ; } # [test] fn err_numeric_hours_too_short () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"+a") . unwrap_err () , @ r###"failed to parse hours in UTC numeric offset "+a": expected two digit hour after sign, but found end of input"### ,) ; } # [test] fn err_numeric_hours_invalid_digits () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"+ab") . unwrap_err () , @ r###"failed to parse hours in UTC numeric offset "+ab": failed to parse "ab" as hours (a two digit integer): invalid digit, expected 0-9 but got a"### ,) ; } # [test] fn err_numeric_hours_out_of_range () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"-26") . unwrap_err () , @ r###"failed to parse hours in UTC numeric offset "-26": offset hours are not valid: parameter 'hours' with value 26 is not in the required range of 0..=25"### ,) ; } # [test] fn err_numeric_minutes_too_short () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"+05:a") . unwrap_err () , @ r###"failed to parse minutes in UTC numeric offset "+05:a": expected two digit minute after hours, but found end of input"### ,) ; } # [test] fn err_numeric_minutes_invalid_digits () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"+05:ab") . unwrap_err () , @ r###"failed to parse minutes in UTC numeric offset "+05:ab": failed to parse "ab" as minutes (a two digit integer): invalid digit, expected 0-9 but got a"### ,) ; } # [test] fn err_numeric_minutes_out_of_range () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"-05:60") . unwrap_err () , @ r###"failed to parse minutes in UTC numeric offset "-05:60": minutes are not valid: parameter 'minutes' with value 60 is not in the required range of 0..=59"### ,) ; } # [test] fn err_numeric_seconds_too_short () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"+05:30:a") . unwrap_err () , @ r###"failed to parse seconds in UTC numeric offset "+05:30:a": expected two digit second after hours, but found end of input"### ,) ; } # [test] fn err_numeric_seconds_invalid_digits () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"+05:30:ab") . unwrap_err () , @ r###"failed to parse seconds in UTC numeric offset "+05:30:ab": failed to parse "ab" as seconds (a two digit integer): invalid digit, expected 0-9 but got a"### ,) ; } # [test] fn err_numeric_seconds_out_of_range () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"-05:30:60") . unwrap_err () , @ r###"failed to parse seconds in UTC numeric offset "-05:30:60": time zone offset seconds are not valid: parameter 'seconds' with value 60 is not in the required range of 0..=59"### ,) ; } # [test] fn err_numeric_fraction_non_empty () { insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"-05:30:44.") . unwrap_err () , @ r###"failed to parse fractional nanoseconds in UTC numeric offset "-05:30:44.": found decimal after seconds component, but did not find any decimal digits after decimal"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"-05:30:44,") . unwrap_err () , @ r###"failed to parse fractional nanoseconds in UTC numeric offset "-05:30:44,": found decimal after seconds component, but did not find any decimal digits after decimal"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"-05:30:44.a") . unwrap_err () , @ r###"failed to parse fractional nanoseconds in UTC numeric offset "-05:30:44.a": found decimal after seconds component, but did not find any decimal digits after decimal"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"-05:30:44,a") . unwrap_err () , @ r###"failed to parse fractional nanoseconds in UTC numeric offset "-05:30:44,a": found decimal after seconds component, but did not find any decimal digits after decimal"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"-053044.a") . unwrap_err () , @ r###"failed to parse fractional nanoseconds in UTC numeric offset "-053044.a": found decimal after seconds component, but did not find any decimal digits after decimal"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse_numeric (b"-053044,a") . unwrap_err () , @ r###"failed to parse fractional nanoseconds in UTC numeric offset "-053044,a": found decimal after seconds component, but did not find any decimal digits after decimal"### ,) ; } # [test] fn err_numeric_subminute_disabled_but_desired () { insta :: assert_snapshot ! (Parser :: new () . subminute (false) . parse_numeric (b"-05:59:32") . unwrap_err () , @ r###"subminute precision for UTC numeric offset "-05:59:32" is not enabled in this context (must provide only integral minutes)"### ,) ; } # [test] fn err_zulu_disabled_but_desired () { insta :: assert_snapshot ! (Parser :: new () . zulu (false) . parse (b"Z") . unwrap_err () , @ r###"found "Z" in "Z" where a numeric UTC offset was expected (this context does not permit the Zulu offset)"### ,) ; insta :: assert_snapshot ! (Parser :: new () . zulu (false) . parse (b"z") . unwrap_err () , @ r###"found "z" in "z" where a numeric UTC offset was expected (this context does not permit the Zulu offset)"### ,) ; } # [test] fn err_numeric_too_big_for_offset () { let numeric = Numeric { sign : t :: Sign :: MAX_SELF , hours : ParsedOffsetHours :: MAX_SELF , minutes : Some (ParsedOffsetMinutes :: MAX_SELF) , seconds : Some (ParsedOffsetSeconds :: MAX_SELF) , nanoseconds : Some (C (499_999_999) . rinto ()) , } ; assert_eq ! (numeric . to_offset () . unwrap () , Offset :: MAX) ; let numeric = Numeric { sign : t :: Sign :: MAX_SELF , hours : ParsedOffsetHours :: MAX_SELF , minutes : Some (ParsedOffsetMinutes :: MAX_SELF) , seconds : Some (ParsedOffsetSeconds :: MAX_SELF) , nanoseconds : Some (C (500_000_000) . rinto ()) , } ; insta :: assert_snapshot ! (numeric . to_offset () . unwrap_err () , @ "due to precision loss, UTC offset '+25:59:59.5' is rounded to a value that is out of bounds: parameter 'offset-seconds' with value 1 is not in the required range of -93599..=93599" ,) ; } # [test] fn err_numeric_too_small_for_offset () { let numeric = Numeric { sign : t :: Sign :: MIN_SELF , hours : ParsedOffsetHours :: MAX_SELF , minutes : Some (ParsedOffsetMinutes :: MAX_SELF) , seconds : Some (ParsedOffsetSeconds :: MAX_SELF) , nanoseconds : Some (C (499_999_999) . rinto ()) , } ; assert_eq ! (numeric . to_offset () . unwrap () , Offset :: MIN) ; let numeric = Numeric { sign : t :: Sign :: MIN_SELF , hours : ParsedOffsetHours :: MAX_SELF , minutes : Some (ParsedOffsetMinutes :: MAX_SELF) , seconds : Some (ParsedOffsetSeconds :: MAX_SELF) , nanoseconds : Some (C (500_000_000) . rinto ()) , } ; insta :: assert_snapshot ! (numeric . to_offset () . unwrap_err () , @ "due to precision loss, UTC offset '-25:59:59.5' is rounded to a value that is out of bounds: parameter 'offset-seconds' with value 1 is not in the required range of -93599..=93599" ,) ; } }
};
}
