// Generated macro for tests (module)
macro_rules! Depcrate_standardtests {
() => {
// Module: crate::standard
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use grep_matcher :: LineTerminator ; use grep_regex :: { RegexMatcher , RegexMatcherBuilder } ; use grep_searcher :: SearcherBuilder ; use termcolor :: { Ansi , NoColor } ; use super :: { ColorSpecs , Standard , StandardBuilder } ; const SHERLOCK : & 'static str = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.\
" ; # [allow (dead_code)] const SHERLOCK_CRLF : & 'static str = "\
For the Doctor Watsons of this world, as opposed to the Sherlock\r
Holmeses, success in the province of detective work must always\r
be, to a very large extent, the result of luck. Sherlock Holmes\r
can extract a clew from a wisp of straw or a flake of cigar ash;\r
but Doctor Watson has to have it taken out for him and dusted,\r
and exhibited clearly, with a label attached.\
" ; fn printer_contents (printer : & mut Standard < NoColor < Vec < u8 > > >) -> String { String :: from_utf8 (printer . get_mut () . get_ref () . to_owned ()) . unwrap () } fn printer_contents_ansi (printer : & mut Standard < Ansi < Vec < u8 > > >) -> String { String :: from_utf8 (printer . get_mut () . get_ref () . to_owned ()) . unwrap () } # [test] fn reports_match () { let matcher = RegexMatcher :: new ("Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; let mut sink = printer . sink (& matcher) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , & mut sink) . unwrap () ; assert ! (sink . has_match ()) ; let matcher = RegexMatcher :: new ("zzzzz") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; let mut sink = printer . sink (& matcher) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , & mut sink) . unwrap () ; assert ! (! sink . has_match ()) ; } # [test] fn reports_binary () { use grep_searcher :: BinaryDetection ; let matcher = RegexMatcher :: new ("Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; let mut sink = printer . sink (& matcher) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , & mut sink) . unwrap () ; assert ! (sink . binary_byte_offset () . is_none ()) ; let matcher = RegexMatcher :: new (".+") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; let mut sink = printer . sink (& matcher) ; SearcherBuilder :: new () . line_number (false) . binary_detection (BinaryDetection :: quit (b'\x00')) . build () . search_reader (& matcher , & b"abc\x00" [..] , & mut sink) . unwrap () ; assert_eq ! (sink . binary_byte_offset () , Some (3)) ; } # [test] fn reports_stats () { use std :: time :: Duration ; let matcher = RegexMatcher :: new ("Sherlock|opposed") . unwrap () ; let mut printer = StandardBuilder :: new () . stats (true) . build (NoColor :: new (vec ! [])) ; let stats = { let mut sink = printer . sink (& matcher) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , & mut sink) . unwrap () ; sink . stats () . unwrap () . clone () } ; let buf = printer_contents (& mut printer) ; assert ! (stats . elapsed () > Duration :: default ()) ; assert_eq ! (stats . searches () , 1) ; assert_eq ! (stats . searches_with_match () , 1) ; assert_eq ! (stats . bytes_searched () , SHERLOCK . len () as u64) ; assert_eq ! (stats . bytes_printed () , buf . len () as u64) ; assert_eq ! (stats . matched_lines () , 2) ; assert_eq ! (stats . matches () , 3) ; } # [test] fn reports_stats_multiple () { use std :: time :: Duration ; let matcher = RegexMatcher :: new ("Sherlock|opposed") . unwrap () ; let mut printer = StandardBuilder :: new () . stats (true) . build (NoColor :: new (vec ! [])) ; let stats = { let mut sink = printer . sink (& matcher) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , & mut sink) . unwrap () ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , & b"zzzzzzzzzz" [..] , & mut sink) . unwrap () ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , & mut sink) . unwrap () ; sink . stats () . unwrap () . clone () } ; let buf = printer_contents (& mut printer) ; assert ! (stats . elapsed () > Duration :: default ()) ; assert_eq ! (stats . searches () , 3) ; assert_eq ! (stats . searches_with_match () , 2) ; assert_eq ! (stats . bytes_searched () , 10 + 2 * SHERLOCK . len () as u64) ; assert_eq ! (stats . bytes_printed () , buf . len () as u64) ; assert_eq ! (stats . matched_lines () , 4) ; assert_eq ! (stats . matches () , 6) ; } # [test] fn context_break () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . separator_context (Some (b"--abc--" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
--abc--
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn context_break_multiple_no_heading () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . separator_search (Some (b"--xyz--" . to_vec ())) . separator_context (Some (b"--abc--" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; SearcherBuilder :: new () . line_number (false) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
--abc--
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
--xyz--
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
--abc--
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn context_break_multiple_heading () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . heading (true) . separator_search (Some (b"--xyz--" . to_vec ())) . separator_context (Some (b"--abc--" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; SearcherBuilder :: new () . line_number (false) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
--abc--
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
--xyz--
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
--abc--
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn path () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . path (false) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "sherlock") ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:For the Doctor Watsons of this world, as opposed to the Sherlock
5:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn separator_field () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . separator_field_match (b"!!" . to_vec ()) . separator_field_context (b"^^" . to_vec ()) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "sherlock") ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
sherlock!!For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock^^Holmeses, success in the province of detective work must always
--
sherlock^^can extract a clew from a wisp of straw or a flake of cigar ash;
sherlock!!but Doctor Watson has to have it taken out for him and dusted,
sherlock^^and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn separator_path () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . separator_path (Some (b'Z')) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "books/sherlock") ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
booksZsherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
booksZsherlock:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn path_terminator () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . path_terminator (Some (b'Z')) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "books/sherlock") ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
books/sherlockZFor the Doctor Watsons of this world, as opposed to the Sherlock
books/sherlockZbut Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn heading () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . heading (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "sherlock") ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
sherlock
For the Doctor Watsons of this world, as opposed to the Sherlock
but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn no_heading () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . heading (false) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "sherlock") ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn no_heading_multiple () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . heading (false) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "sherlock") ,) . unwrap () ; let matcher = RegexMatcher :: new ("Sherlock") . unwrap () ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "sherlock") ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:but Doctor Watson has to have it taken out for him and dusted,
sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
" ; assert_eq_printed ! (expected , got) ; } # [test] fn heading_multiple () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . heading (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "sherlock") ,) . unwrap () ; let matcher = RegexMatcher :: new ("Sherlock") . unwrap () ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink_with_path (& matcher , "sherlock") ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
sherlock
For the Doctor Watsons of this world, as opposed to the Sherlock
but Doctor Watson has to have it taken out for him and dusted,
sherlock
For the Doctor Watsons of this world, as opposed to the Sherlock
be, to a very large extent, the result of luck. Sherlock Holmes
" ; assert_eq_printed ! (expected , got) ; } # [test] fn trim_ascii () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . trim_ascii (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , "   Watson" . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
Watson
" ; assert_eq_printed ! (expected , got) ; } # [test] fn trim_ascii_multi_line () { let matcher = RegexMatcher :: new ("(?s:.{0})Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . trim_ascii (true) . stats (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . multi_line (true) . build () . search_reader (& matcher , "   Watson" . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
Watson
" ; assert_eq_printed ! (expected , got) ; } # [test] fn trim_ascii_with_line_term () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . trim_ascii (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . before_context (1) . build () . search_reader (& matcher , "\n   Watson" . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1-
2:Watson
" ; assert_eq_printed ! (expected , got) ; } # [test] fn line_number () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:For the Doctor Watsons of this world, as opposed to the Sherlock
5:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn line_number_multi_line () { let matcher = RegexMatcher :: new ("(?s)Watson.+Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . multi_line (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:For the Doctor Watsons of this world, as opposed to the Sherlock
2:Holmeses, success in the province of detective work must always
3:be, to a very large extent, the result of luck. Sherlock Holmes
4:can extract a clew from a wisp of straw or a flake of cigar ash;
5:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn column_number () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
16:For the Doctor Watsons of this world, as opposed to the Sherlock
12:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn column_number_multi_line () { let matcher = RegexMatcher :: new ("(?s)Watson.+Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . multi_line (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
16:For the Doctor Watsons of this world, as opposed to the Sherlock
16:Holmeses, success in the province of detective work must always
16:be, to a very large extent, the result of luck. Sherlock Holmes
16:can extract a clew from a wisp of straw or a flake of cigar ash;
16:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn byte_offset () { let matcher = RegexMatcher :: new ("Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . byte_offset (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
258:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn byte_offset_multi_line () { let matcher = RegexMatcher :: new ("(?s)Watson.+Watson") . unwrap () ; let mut printer = StandardBuilder :: new () . byte_offset (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . multi_line (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65:Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes
193:can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_columns () { let matcher = RegexMatcher :: new ("ash|dusted") . unwrap () ; let mut printer = StandardBuilder :: new () . max_columns (Some (63)) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
[Omitted long matching line]
but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_columns_preview () { let matcher = RegexMatcher :: new ("exhibited|dusted") . unwrap () ; let mut printer = StandardBuilder :: new () . max_columns (Some (46)) . max_columns_preview (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
but Doctor Watson has to have it taken out for [... omitted end of long line]
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_columns_with_count () { let matcher = RegexMatcher :: new ("cigar|ash|dusted") . unwrap () ; let mut printer = StandardBuilder :: new () . stats (true) . max_columns (Some (63)) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
[Omitted long line with 2 matches]
but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_columns_with_count_preview_no_match () { let matcher = RegexMatcher :: new ("exhibited|has to have it") . unwrap () ; let mut printer = StandardBuilder :: new () . stats (true) . max_columns (Some (46)) . max_columns_preview (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
but Doctor Watson has to have it taken out for [... 0 more matches]
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_columns_with_count_preview_one_match () { let matcher = RegexMatcher :: new ("exhibited|dusted") . unwrap () ; let mut printer = StandardBuilder :: new () . stats (true) . max_columns (Some (46)) . max_columns_preview (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
but Doctor Watson has to have it taken out for [... 1 more match]
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_columns_with_count_preview_two_matches () { let matcher = RegexMatcher :: new ("exhibited|dusted|has to have it") . unwrap () ; let mut printer = StandardBuilder :: new () . stats (true) . max_columns (Some (46)) . max_columns_preview (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
but Doctor Watson has to have it taken out for [... 1 more match]
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_columns_multi_line () { let matcher = RegexMatcher :: new ("(?s)ash.+dusted") . unwrap () ; let mut printer = StandardBuilder :: new () . max_columns (Some (63)) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . multi_line (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
[Omitted long matching line]
but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_columns_multi_line_preview () { let matcher = RegexMatcher :: new ("(?s)clew|cigar ash.+have it|exhibited") . unwrap () ; let mut printer = StandardBuilder :: new () . stats (true) . max_columns (Some (46)) . max_columns_preview (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . multi_line (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
can extract a clew from a wisp of straw or a f [... 1 more match]
but Doctor Watson has to have it taken out for [... 0 more matches]
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_matches () { let matcher = RegexMatcher :: new ("Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . max_matches (Some (1)) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_matches_context () { let matcher = RegexMatcher :: new ("Doctor Watsons") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . max_matches (Some (1)) . line_number (false) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
" ; assert_eq_printed ! (expected , got) ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . max_matches (Some (1)) . line_number (false) . after_context (4) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; let matcher = RegexMatcher :: new ("Doctor Watsons|but Doctor") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . max_matches (Some (2)) . line_number (false) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
--
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . max_matches (Some (2)) . line_number (false) . after_context (4) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_matches_context_invert () { let matcher = RegexMatcher :: new ("success|extent|clew|dusted|exhibited") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . invert_match (true) . max_matches (Some (1)) . line_number (false) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
" ; assert_eq_printed ! (expected , got) ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . invert_match (true) . max_matches (Some (1)) . line_number (false) . after_context (4) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; let matcher = RegexMatcher :: new ("success|extent|clew|exhibited") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . invert_match (true) . max_matches (Some (2)) . line_number (false) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
--
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . invert_match (true) . max_matches (Some (2)) . line_number (false) . after_context (4) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_matches_multi_line1 () { let matcher = RegexMatcher :: new ("(?s:.{0})Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . multi_line (true) . max_matches (Some (1)) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_matches_multi_line2 () { let matcher = RegexMatcher :: new (r"(?s)Watson.+?(Holmeses|clearly)") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . multi_line (true) . max_matches (Some (1)) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_matches_multi_line3 () { let matcher = RegexMatcher :: new (r"line 2\nline 3") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . multi_line (true) . max_matches (Some (1)) . build () . search_reader (& matcher , "line 2\nline 3 x\nline 2\nline 3\n" . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
line 2
line 3 x
" ; assert_eq_printed ! (expected , got) ; } # [test] fn max_matches_multi_line4 () { let matcher = RegexMatcher :: new (r"line 2\nline 3|x\nline 2\n") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . multi_line (true) . max_matches (Some (1)) . build () . search_reader (& matcher , "line 2\nline 3 x\nline 2\nline 3 x\n" . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
line 2
line 3 x
" ; assert_eq_printed ! (expected , got) ; } # [test] fn only_matching () { let matcher = RegexMatcher :: new ("Doctor Watsons|Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:9:Doctor Watsons
1:57:Sherlock
3:49:Sherlock
" ; assert_eq_printed ! (expected , got) ; } # [test] fn only_matching_multi_line1 () { let matcher = RegexMatcher :: new (r"(?s:.{0})(Doctor Watsons|Sherlock)") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:9:Doctor Watsons
1:57:Sherlock
3:49:Sherlock
" ; assert_eq_printed ! (expected , got) ; } # [test] fn only_matching_multi_line2 () { let matcher = RegexMatcher :: new (r"(?s)Watson.+?(Holmeses|clearly)") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:16:Watsons of this world, as opposed to the Sherlock
2:16:Holmeses
5:12:Watson has to have it taken out for him and dusted,
6:12:and exhibited clearly
" ; assert_eq_printed ! (expected , got) ; } # [test] fn only_matching_max_columns () { let matcher = RegexMatcher :: new ("Doctor Watsons|Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . max_columns (Some (10)) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:9:[Omitted long matching line]
1:57:Sherlock
3:49:Sherlock
" ; assert_eq_printed ! (expected , got) ; } # [test] fn only_matching_max_columns_preview () { let matcher = RegexMatcher :: new ("Doctor Watsons|Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . max_columns (Some (10)) . max_columns_preview (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:9:Doctor Wat [... 0 more matches]
1:57:Sherlock
3:49:Sherlock
" ; assert_eq_printed ! (expected , got) ; } # [test] fn only_matching_max_columns_multi_line1 () { let matcher = RegexMatcher :: new (r"(?s:.{0})(Doctor Watsons|Sherlock)") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . max_columns (Some (10)) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:9:[Omitted long matching line]
1:57:Sherlock
3:49:Sherlock
" ; assert_eq_printed ! (expected , got) ; } # [test] fn only_matching_max_columns_preview_multi_line1 () { let matcher = RegexMatcher :: new (r"(?s:.{0})(Doctor Watsons|Sherlock)") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . max_columns (Some (10)) . max_columns_preview (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:9:Doctor Wat [... 0 more matches]
1:57:Sherlock
3:49:Sherlock
" ; assert_eq_printed ! (expected , got) ; } # [test] fn only_matching_max_columns_multi_line2 () { let matcher = RegexMatcher :: new (r"(?s)Watson.+?(Holmeses|clearly)") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . max_columns (Some (50)) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:16:Watsons of this world, as opposed to the Sherlock
2:16:Holmeses
5:12:[Omitted long matching line]
6:12:and exhibited clearly
" ; assert_eq_printed ! (expected , got) ; } # [test] fn only_matching_max_columns_preview_multi_line2 () { let matcher = RegexMatcher :: new (r"(?s)Watson.+?(Holmeses|clearly)") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . max_columns (Some (50)) . max_columns_preview (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:16:Watsons of this world, as opposed to the Sherlock
2:16:Holmeses
5:12:Watson has to have it taken out for him and dusted [... 0 more matches]
6:12:and exhibited clearly
" ; assert_eq_printed ! (expected , got) ; } # [test] fn per_match () { let matcher = RegexMatcher :: new ("Doctor Watsons|Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . per_match (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:9:For the Doctor Watsons of this world, as opposed to the Sherlock
1:57:For the Doctor Watsons of this world, as opposed to the Sherlock
3:49:be, to a very large extent, the result of luck. Sherlock Holmes
" ; assert_eq_printed ! (expected , got) ; } # [test] fn per_match_multi_line1 () { let matcher = RegexMatcher :: new (r"(?s:.{0})(Doctor Watsons|Sherlock)") . unwrap () ; let mut printer = StandardBuilder :: new () . per_match (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:9:For the Doctor Watsons of this world, as opposed to the Sherlock
1:57:For the Doctor Watsons of this world, as opposed to the Sherlock
3:49:be, to a very large extent, the result of luck. Sherlock Holmes
" ; assert_eq_printed ! (expected , got) ; } # [test] fn per_match_multi_line2 () { let matcher = RegexMatcher :: new (r"(?s)Watson.+?(Holmeses|clearly)") . unwrap () ; let mut printer = StandardBuilder :: new () . per_match (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:16:For the Doctor Watsons of this world, as opposed to the Sherlock
2:1:Holmeses, success in the province of detective work must always
5:12:but Doctor Watson has to have it taken out for him and dusted,
6:1:and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn per_match_multi_line3 () { let matcher = RegexMatcher :: new (r"(?s)Watson.+?Holmeses|always.+?be") . unwrap () ; let mut printer = StandardBuilder :: new () . per_match (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:16:For the Doctor Watsons of this world, as opposed to the Sherlock
2:1:Holmeses, success in the province of detective work must always
2:58:Holmeses, success in the province of detective work must always
3:1:be, to a very large extent, the result of luck. Sherlock Holmes
" ; assert_eq_printed ! (expected , got) ; } # [test] fn per_match_multi_line1_only_first_line () { let matcher = RegexMatcher :: new (r"(?s:.{0})(Doctor Watsons|Sherlock)") . unwrap () ; let mut printer = StandardBuilder :: new () . per_match (true) . per_match_one_line (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:9:For the Doctor Watsons of this world, as opposed to the Sherlock
1:57:For the Doctor Watsons of this world, as opposed to the Sherlock
3:49:be, to a very large extent, the result of luck. Sherlock Holmes
" ; assert_eq_printed ! (expected , got) ; } # [test] fn per_match_multi_line2_only_first_line () { let matcher = RegexMatcher :: new (r"(?s)Watson.+?(Holmeses|clearly)") . unwrap () ; let mut printer = StandardBuilder :: new () . per_match (true) . per_match_one_line (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:16:For the Doctor Watsons of this world, as opposed to the Sherlock
5:12:but Doctor Watson has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn per_match_multi_line3_only_first_line () { let matcher = RegexMatcher :: new (r"(?s)Watson.+?Holmeses|always.+?be") . unwrap () ; let mut printer = StandardBuilder :: new () . per_match (true) . per_match_one_line (true) . column (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:16:For the Doctor Watsons of this world, as opposed to the Sherlock
2:58:Holmeses, success in the province of detective work must always
" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement_passthru () { let matcher = RegexMatcher :: new (r"Sherlock|Doctor (\w+)") . unwrap () ; let mut printer = StandardBuilder :: new () . replacement (Some (b"doctah $1 MD" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . passthru (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:For the doctah Watsons MD of this world, as opposed to the doctah  MD
2-Holmeses, success in the province of detective work must always
3:be, to a very large extent, the result of luck. doctah  MD Holmes
4-can extract a clew from a wisp of straw or a flake of cigar ash;
5:but doctah Watson MD has to have it taken out for him and dusted,
6-and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement () { let matcher = RegexMatcher :: new (r"Sherlock|Doctor (\w+)") . unwrap () ; let mut printer = StandardBuilder :: new () . replacement (Some (b"doctah $1 MD" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:For the doctah Watsons MD of this world, as opposed to the doctah  MD
3:be, to a very large extent, the result of luck. doctah  MD Holmes
5:but doctah Watson MD has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement_multi_line () { let matcher = RegexMatcher :: new (r"\n") . unwrap () ; let mut printer = StandardBuilder :: new () . replacement (Some (b"?" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . multi_line (true) . build () . search_reader (& matcher , "hello\nworld\n" . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "1:hello?world?\n" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement_multi_line_diff_line_term () { let matcher = RegexMatcherBuilder :: new () . line_terminator (Some (b'\x00')) . build (r"\n") . unwrap () ; let mut printer = StandardBuilder :: new () . replacement (Some (b"?" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_terminator (LineTerminator :: byte (b'\x00')) . line_number (true) . multi_line (true) . build () . search_reader (& matcher , "hello\nworld\n" . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "1:hello?world?\x00" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement_multi_line_combine_lines () { let matcher = RegexMatcher :: new (r"\n(.)?") . unwrap () ; let mut printer = StandardBuilder :: new () . replacement (Some (b"?$1" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . multi_line (true) . build () . search_reader (& matcher , "hello\nworld\n" . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "1:hello?world?\n" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement_max_columns () { let matcher = RegexMatcher :: new (r"Sherlock|Doctor (\w+)") . unwrap () ; let mut printer = StandardBuilder :: new () . max_columns (Some (67)) . replacement (Some (b"doctah $1 MD" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:[Omitted long line with 2 matches]
3:be, to a very large extent, the result of luck. doctah  MD Holmes
5:but doctah Watson MD has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement_max_columns_preview1 () { let matcher = RegexMatcher :: new (r"Sherlock|Doctor (\w+)") . unwrap () ; let mut printer = StandardBuilder :: new () . max_columns (Some (67)) . max_columns_preview (true) . replacement (Some (b"doctah $1 MD" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:For the doctah Watsons MD of this world, as opposed to the doctah   [... 0 more matches]
3:be, to a very large extent, the result of luck. doctah  MD Holmes
5:but doctah Watson MD has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement_max_columns_preview2 () { let matcher = RegexMatcher :: new ("exhibited|dusted|has to have it") . unwrap () ; let mut printer = StandardBuilder :: new () . max_columns (Some (43)) . max_columns_preview (true) . replacement (Some (b"xxx" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (false) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
but Doctor Watson xxx taken out for him and [... 1 more match]
and xxx clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement_only_matching () { let matcher = RegexMatcher :: new (r"Sherlock|Doctor (\w+)") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . replacement (Some (b"doctah $1 MD" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:doctah Watsons MD
1:doctah  MD
3:doctah  MD
5:doctah Watson MD
" ; assert_eq_printed ! (expected , got) ; } # [test] fn replacement_per_match () { let matcher = RegexMatcher :: new (r"Sherlock|Doctor (\w+)") . unwrap () ; let mut printer = StandardBuilder :: new () . per_match (true) . replacement (Some (b"doctah $1 MD" . to_vec ())) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1:For the doctah Watsons MD of this world, as opposed to the doctah  MD
1:For the doctah Watsons MD of this world, as opposed to the doctah  MD
3:be, to a very large extent, the result of luck. doctah  MD Holmes
5:but doctah Watson MD has to have it taken out for him and dusted,
" ; assert_eq_printed ! (expected , got) ; } # [test] fn invert () { let matcher = RegexMatcher :: new (r"Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . invert_match (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
2:Holmeses, success in the province of detective work must always
4:can extract a clew from a wisp of straw or a flake of cigar ash;
5:but Doctor Watson has to have it taken out for him and dusted,
6:and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn invert_multi_line () { let matcher = RegexMatcher :: new (r"(?s:.{0})Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . invert_match (true) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
2:Holmeses, success in the province of detective work must always
4:can extract a clew from a wisp of straw or a flake of cigar ash;
5:but Doctor Watson has to have it taken out for him and dusted,
6:and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn invert_context () { let matcher = RegexMatcher :: new (r"Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . invert_match (true) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1-For the Doctor Watsons of this world, as opposed to the Sherlock
2:Holmeses, success in the province of detective work must always
3-be, to a very large extent, the result of luck. Sherlock Holmes
4:can extract a clew from a wisp of straw or a flake of cigar ash;
5:but Doctor Watson has to have it taken out for him and dusted,
6:and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn invert_context_multi_line () { let matcher = RegexMatcher :: new (r"(?s:.{0})Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . invert_match (true) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1-For the Doctor Watsons of this world, as opposed to the Sherlock
2:Holmeses, success in the province of detective work must always
3-be, to a very large extent, the result of luck. Sherlock Holmes
4:can extract a clew from a wisp of straw or a flake of cigar ash;
5:but Doctor Watson has to have it taken out for him and dusted,
6:and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn invert_context_only_matching () { let matcher = RegexMatcher :: new (r"Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . line_number (true) . invert_match (true) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1-Sherlock
2:Holmeses, success in the province of detective work must always
3-Sherlock
4:can extract a clew from a wisp of straw or a flake of cigar ash;
5:but Doctor Watson has to have it taken out for him and dusted,
6:and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn invert_context_only_matching_multi_line () { let matcher = RegexMatcher :: new (r"(?s:.{0})Sherlock") . unwrap () ; let mut printer = StandardBuilder :: new () . only_matching (true) . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . multi_line (true) . line_number (true) . invert_match (true) . before_context (1) . after_context (1) . build () . search_reader (& matcher , SHERLOCK . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "\
1-Sherlock
2:Holmeses, success in the province of detective work must always
3-Sherlock
4:can extract a clew from a wisp of straw or a flake of cigar ash;
5:but Doctor Watson has to have it taken out for him and dusted,
6:and exhibited clearly, with a label attached.
" ; assert_eq_printed ! (expected , got) ; } # [test] fn regression_search_empty_with_crlf () { let matcher = RegexMatcherBuilder :: new () . crlf (true) . build (r"x?") . unwrap () ; let mut printer = StandardBuilder :: new () . color_specs (ColorSpecs :: default_with_color ()) . build (Ansi :: new (vec ! [])) ; SearcherBuilder :: new () . line_terminator (LineTerminator :: crlf ()) . build () . search_reader (& matcher , & b"\n" [..] , printer . sink (& matcher)) . unwrap () ; let got = printer_contents_ansi (& mut printer) ; assert ! (! got . is_empty ()) ; } # [test] fn regression_after_context_with_match () { let haystack = "\
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
" ; let matcher = RegexMatcherBuilder :: new () . build (r"d") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; SearcherBuilder :: new () . max_matches (Some (1)) . line_number (true) . after_context (2) . build () . search_reader (& matcher , haystack . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "4:d\n5-e\n6:d\n" ; assert_eq_printed ! (expected , got) ; } # [test] fn regression_crlf_preserve () { let haystack = "hello\nworld\r\n" ; let matcher = RegexMatcherBuilder :: new () . crlf (true) . build (r".") . unwrap () ; let mut printer = StandardBuilder :: new () . build (NoColor :: new (vec ! [])) ; let mut searcher = SearcherBuilder :: new () . line_number (false) . line_terminator (LineTerminator :: crlf ()) . build () ; searcher . search_reader (& matcher , haystack . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "hello\nworld\r\n" ; assert_eq_printed ! (expected , got) ; let mut printer = StandardBuilder :: new () . replacement (Some (b"$0" . to_vec ())) . build (NoColor :: new (vec ! [])) ; searcher . search_reader (& matcher , haystack . as_bytes () , printer . sink (& matcher) ,) . unwrap () ; let got = printer_contents (& mut printer) ; let expected = "hello\nworld\r\n" ; assert_eq_printed ! (expected , got) ; } }
};
}
