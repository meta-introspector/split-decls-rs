macro_rules! deps {
    () => {
        AhoCorasick!();
        MatchKind!();
    };
}

macro_rules! stream_not_allowed_leftmost_longest {
    () => {
        deps!();
        # [cfg (feature = "std")] # [test] # [should_panic] fn stream_not_allowed_leftmost_longest () { let fsm = AhoCorasick :: builder () . match_kind (MatchKind :: LeftmostLongest) . build (None :: < String >) . unwrap () ; assert_eq ! (fsm . stream_find_iter (& b"" [..]) . count () , 0) ; }
    };
}

stream_not_allowed_leftmost_longest!()