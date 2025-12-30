// Generated macro for FieldParser (struct)
macro_rules! Depcrate_tz_zicFieldParser {
() => {
// Module: crate::tz::zic
// Provides: {"FieldParser"}
// Dependencies: {}
# [doc = " A parser that emits lines as sequences of fields."] # [doc = ""] # [doc = " It is responsible for managing the state regarding whether to expect a"] # [doc = " zone continuation line or not. It also knows to skip empty or commented"] # [doc = " out lines."] struct FieldParser < 'a > { # [doc = " The full underlying source data as an iterator of lines."] lines : core :: str :: Lines < 'a > , # [doc = " The current line number, starting at 1."] line_number : usize , # [doc = " The fields from the current line, initially empty."] fields : Vec < & 'a str > , # [doc = " Set to the name of the zone when a continuation line for that zone is"] # [doc = " expected."] continuation_zone_for : Option < String > , }
};
}
