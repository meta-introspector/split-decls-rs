macro_rules! deps {
    () => {
        AhoCorasick!();
        Input!();
        MatchKind!();
    };
}

macro_rules! prefilter_stays_in_bounds {
    () => {
        deps!();
        # [test] fn prefilter_stays_in_bounds () { let ac = AhoCorasick :: builder () . match_kind (MatchKind :: LeftmostFirst) . build (& ["sam" , "frodo" , "pippin" , "merry" , "gandalf" , "sauron"]) . unwrap () ; let haystack = "foo gandalf" ; assert_eq ! (None , ac . find (Input :: new (haystack) . range (0 .. 10))) ; }
    };
}

prefilter_stays_in_bounds!();