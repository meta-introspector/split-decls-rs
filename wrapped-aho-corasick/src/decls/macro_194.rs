macro_rules! deps {
    () => {
        Config!();
        MatchKind!();
    };
}

macro_rules! macro_194 {
    () => {
        deps!();
        testconfig ! (search_teddy_leftmost_longest , PACKED_LEFTMOST_LONGEST , | c : & mut Config | { c . only_teddy (true) . match_kind (MatchKind :: LeftmostLongest) ; }) ;
    };
}

macro_194!()