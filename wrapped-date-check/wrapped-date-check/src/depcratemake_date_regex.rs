// Generated macro for make_date_regex (function)
macro_rules! Depcratemake_date_regex {
() => {
// Module: crate
// Provides: {"make_date_regex"}
// Dependencies: {}
fn make_date_regex () -> Regex { Regex :: new (r"(?x) # insignificant whitespace mode
        (<!--\s*
          date-check:\s*
          (?P<m1>[[:alpha:]]+)\s+
          (?P<y1>\d{4})\s*-->
        )
        |
        (<!--\s*
          date-check\s*-->\s+
          (?P<m2>[[:alpha:]]+)\s+
          (?P<y2>\d{4})\b
        )
    " ,) . unwrap () }
};
}
