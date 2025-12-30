// Generated macro for tests (module)
macro_rules! Depcrate_jsontests {
() => {
// Module: crate::json
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use grep_matcher :: LineTerminator ; use grep_regex :: { RegexMatcher , RegexMatcherBuilder } ; use grep_searcher :: SearcherBuilder ; use super :: { JSON , JSONBuilder } ; const SHERLOCK : & 'static [u8] = b"\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
" ; fn printer_contents (printer : & mut JSON < Vec < u8 > >) -> String { String :: from_utf8 (printer . get_mut () . to_owned ()) . unwrap () } # [test] fn binary_detection () { use grep_searcher :: BinaryDetection ; const BINARY : & 'static [u8] = b"\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew \x00 from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.\
" ; let matcher = RegexMatcher :: new (r"Watson") . unwrap () ; let mut printer = JSONBuilder :: new () . build (vec ! []) ; SearcherBuilder :: new () . binary_detection (BinaryDetection :: quit (b'\x00')) . heap_limit (Some (80)) . build () . search_reader (& matcher , BINARY , printer . sink (& matcher)) . unwrap () ; let got = printer_contents (& mut printer) ; assert_eq ! (got . lines () . count () , 3) ; let last = got . lines () . last () . unwrap () ; assert ! (last . contains (r#""binary_offset":212,"#)) ; } # [test] fn max_matches () { let matcher = RegexMatcher :: new (r"Watson") . unwrap () ; let mut printer = JSONBuilder :: new () . build (vec ! []) ; SearcherBuilder :: new () . max_matches (Some (1)) . build () . search_reader (& matcher , SHERLOCK , printer . sink (& matcher)) . unwrap () ; let got = printer_contents (& mut printer) ; assert_eq ! (got . lines () . count () , 3) ; } # [test] fn max_matches_after_context () { let haystack = "\
a
b
c
d
e
d
e
d
e
d
e
" ; let matcher = RegexMatcher :: new (r"d") . unwrap () ; let mut printer = JSONBuilder :: new () . build (vec ! []) ; SearcherBuilder :: new () . after_context (2) . max_matches (Some (1)) . build () . search_reader (& matcher , haystack . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; assert_eq ! (got . lines () . count () , 5) ; } # [test] fn no_match () { let matcher = RegexMatcher :: new (r"DOES NOT MATCH") . unwrap () ; let mut printer = JSONBuilder :: new () . build (vec ! []) ; SearcherBuilder :: new () . build () . search_reader (& matcher , SHERLOCK , printer . sink (& matcher)) . unwrap () ; let got = printer_contents (& mut printer) ; assert ! (got . is_empty ()) ; } # [test] fn always_begin_end_no_match () { let matcher = RegexMatcher :: new (r"DOES NOT MATCH") . unwrap () ; let mut printer = JSONBuilder :: new () . always_begin_end (true) . build (vec ! []) ; SearcherBuilder :: new () . build () . search_reader (& matcher , SHERLOCK , printer . sink (& matcher)) . unwrap () ; let got = printer_contents (& mut printer) ; assert_eq ! (got . lines () . count () , 2) ; assert ! (got . contains ("begin") && got . contains ("end")) ; } # [test] fn missing_crlf () { let haystack = "test\r\n" . as_bytes () ; let matcher = RegexMatcherBuilder :: new () . build ("test") . unwrap () ; let mut printer = JSONBuilder :: new () . build (vec ! []) ; SearcherBuilder :: new () . build () . search_reader (& matcher , haystack , printer . sink (& matcher)) . unwrap () ; let got = printer_contents (& mut printer) ; assert_eq ! (got . lines () . count () , 3) ; assert ! (got . lines () . nth (1) . unwrap () . contains (r"test\r\n") , r"missing 'test\r\n' in '{}'" , got . lines () . nth (1) . unwrap () ,) ; let matcher = RegexMatcherBuilder :: new () . crlf (true) . build ("test") . unwrap () ; let mut printer = JSONBuilder :: new () . build (vec ! []) ; SearcherBuilder :: new () . line_terminator (LineTerminator :: crlf ()) . build () . search_reader (& matcher , haystack , printer . sink (& matcher)) . unwrap () ; let got = printer_contents (& mut printer) ; assert_eq ! (got . lines () . count () , 3) ; assert ! (got . lines () . nth (1) . unwrap () . contains (r"test\r\n") , r"missing 'test\r\n' in '{}'" , got . lines () . nth (1) . unwrap () ,) ; } }
};
}
