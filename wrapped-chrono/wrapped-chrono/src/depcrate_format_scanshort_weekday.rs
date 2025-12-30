// Generated macro for short_weekday (function)
macro_rules! Depcrate_format_scanshort_weekday {
() => {
// Module: crate::format::scan
// Provides: {"short_weekday"}
// Dependencies: {}
# [doc = " Tries to parse the weekday with the first three ASCII letters."] pub (super) fn short_weekday (s : & str) -> ParseResult < (& str , Weekday) > { if s . len () < 3 { return Err (TOO_SHORT) ; } let buf = s . as_bytes () ; let weekday = match (buf [0] | 32 , buf [1] | 32 , buf [2] | 32) { (b'm' , b'o' , b'n') => Weekday :: Mon , (b't' , b'u' , b'e') => Weekday :: Tue , (b'w' , b'e' , b'd') => Weekday :: Wed , (b't' , b'h' , b'u') => Weekday :: Thu , (b'f' , b'r' , b'i') => Weekday :: Fri , (b's' , b'a' , b't') => Weekday :: Sat , (b's' , b'u' , b'n') => Weekday :: Sun , _ => return Err (INVALID) , } ; Ok ((& s [3 ..] , weekday)) }
};
}
