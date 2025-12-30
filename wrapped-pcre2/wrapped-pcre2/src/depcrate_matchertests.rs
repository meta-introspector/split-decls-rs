// Generated macro for tests (module)
macro_rules! Depcrate_matchertests {
() => {
// Module: crate::matcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use grep_matcher :: LineMatchKind ; use super :: * ; # [test] fn word () { let matcher = RegexMatcherBuilder :: new () . word (true) . build (r"-2") . unwrap () ; assert ! (matcher . is_match (b"abc -2 foo") . unwrap ()) ; let matcher = RegexMatcherBuilder :: new () . word (false) . build (r"\b-2\b") . unwrap () ; assert ! (! matcher . is_match (b"abc -2 foo") . unwrap ()) ; } # [test] fn line_terminator_crlf () { let matcher = RegexMatcherBuilder :: new () . multi_line (true) . build (r"abc$") . unwrap () ; assert ! (matcher . is_match (b"abc\n") . unwrap ()) ; let matcher = RegexMatcherBuilder :: new () . multi_line (true) . build (r"abc$") . unwrap () ; assert ! (! matcher . is_match (b"abc\r\n") . unwrap ()) ; let matcher = RegexMatcherBuilder :: new () . multi_line (true) . crlf (true) . build (r"abc$") . unwrap () ; assert ! (matcher . is_match (b"abc\r\n") . unwrap ()) ; } # [test] fn case_smart () { let matcher = RegexMatcherBuilder :: new () . case_smart (true) . build (r"abc") . unwrap () ; assert ! (matcher . is_match (b"ABC") . unwrap ()) ; let matcher = RegexMatcherBuilder :: new () . case_smart (true) . build (r"aBc") . unwrap () ; assert ! (! matcher . is_match (b"ABC") . unwrap ()) ; } # [test] fn candidate_lines () { fn is_confirmed (m : LineMatchKind) -> bool { match m { LineMatchKind :: Confirmed (_) => true , _ => false , } } let matcher = RegexMatcherBuilder :: new () . build (r"\wfoo\s") . unwrap () ; let m = matcher . find_candidate_line (b"afoo ") . unwrap () . unwrap () ; assert ! (is_confirmed (m)) ; } }
};
}
