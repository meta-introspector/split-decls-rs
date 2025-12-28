macro_rules! deps {
    () => {
        AhoCorasick!();
        MatchKind!();
    };
}

macro_rules! overlapping_not_allowed_leftmost_first {
    () => {
        deps!();
        # [test] # [should_panic] fn overlapping_not_allowed_leftmost_first () { let fsm = AhoCorasick :: builder () . match_kind (MatchKind :: LeftmostFirst) . build (None :: < String >) . unwrap () ; assert_eq ! (fsm . find_overlapping_iter ("") . count () , 0) ; }
    };
}

overlapping_not_allowed_leftmost_first!()