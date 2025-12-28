macro_rules! deps {
    () => {
        WildStr!();
    };
}

macro_rules! match_contains {
    () => {
        deps!();
        # [doc = " Checks that the given string contains the given contiguous lines"] # [doc = " somewhere."] # [doc = ""] # [doc = " See [Patterns](index.html#patterns) for more information on pattern matching."] pub (crate) fn match_contains (expected : & str , actual : & str , redactions : & snapbox :: Redactions ,) -> Result < () > { let expected = normalize_expected (expected , redactions) ; let actual = normalize_actual (actual , redactions) ; let e : Vec < _ > = expected . lines () . map (| line | WildStr :: new (line)) . collect () ; let a : Vec < _ > = actual . lines () . collect () ; if e . len () == 0 { bail ! ("expected length must not be zero") ; } for window in a . windows (e . len ()) { if e == window { return Ok (()) ; } } bail ! ("expected to find:\n\
         {}\n\n\
         did not find in output:\n\
         {}" , expected , actual) ; }
    };
}

match_contains!();