macro_rules! match_does_not_contain {
    () => {
        # [doc = " Checks that the given string does not contain the given contiguous lines"] # [doc = " anywhere."] # [doc = ""] # [doc = " See [Patterns](index.html#patterns) for more information on pattern matching."] pub (crate) fn match_does_not_contain (expected : & str , actual : & str , redactions : & snapbox :: Redactions ,) -> Result < () > { if match_contains (expected , actual , redactions) . is_ok () { bail ! ("expected not to find:\n\
             {}\n\n\
             but found in output:\n\
             {}" , expected , actual) ; } else { Ok (()) } }
    };
}

match_does_not_contain!();