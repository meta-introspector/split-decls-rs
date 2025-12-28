macro_rules! deps {
    () => {
        AhoCorasick!();
        MatchKind!();
    };
}

macro_rules! overlapping_not_allowed_leftmost_longest {
    () => {
        deps!();
        # [test] # [should_panic] fn overlapping_not_allowed_leftmost_longest () { let fsm = AhoCorasick :: builder () . match_kind (MatchKind :: LeftmostLongest) . build (None :: < String >) . unwrap () ; assert_eq ! (fsm . find_overlapping_iter ("") . count () , 0) ; }
    };
}

overlapping_not_allowed_leftmost_longest!();