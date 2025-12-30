// Generated macro for match_with_without (function)
macro_rules! Depcrate_comparematch_with_without {
() => {
// Module: crate::compare
// Provides: {"match_with_without"}
// Dependencies: {}
# [doc = " Checks that the given string has a line that contains the given patterns,"] # [doc = " and that line also does not contain the `without` patterns."] # [doc = ""] # [doc = " See [Patterns](index.html#patterns) for more information on pattern matching."] # [doc = ""] # [doc = " See [`crate::Execs::with_stderr_line_without`] for an example and cautions"] # [doc = " against using."] pub (crate) fn match_with_without (actual : & str , with : & [String] , without : & [String] , redactions : & snapbox :: Redactions ,) -> Result < () > { let actual = normalize_actual (actual , redactions) ; let norm = | s : & String | format ! ("[..]{}[..]" , normalize_expected (s , redactions)) ; let with : Vec < _ > = with . iter () . map (norm) . collect () ; let without : Vec < _ > = without . iter () . map (norm) . collect () ; let with_wild : Vec < _ > = with . iter () . map (| w | WildStr :: new (w)) . collect () ; let without_wild : Vec < _ > = without . iter () . map (| w | WildStr :: new (w)) . collect () ; let matches : Vec < _ > = actual . lines () . filter (| line | with_wild . iter () . all (| with | with == line)) . filter (| line | ! without_wild . iter () . any (| without | without == line)) . collect () ; match matches . len () { 0 => bail ! ("Could not find expected line in output.\n\
             With contents: {:?}\n\
             Without contents: {:?}\n\
             Actual stderr:\n\
             {}\n" , with , without , actual) , 1 => Ok (()) , _ => bail ! ("Found multiple matching lines, but only expected one.\n\
             With contents: {:?}\n\
             Without contents: {:?}\n\
             Matching lines:\n\
             {}\n" , with , without , itertools :: join (matches , "\n")) , } }
};
}
