macro_rules! deps {
    () => {
        MatchPosition!();
    };
}

macro_rules! FindReducer {
    () => {
        deps!();
        struct FindReducer { match_position : MatchPosition , }
    };
}

FindReducer!()