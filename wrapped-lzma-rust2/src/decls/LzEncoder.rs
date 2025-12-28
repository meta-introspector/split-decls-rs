macro_rules! deps {
    () => {
        Matches!();
        LzEncoderData!();
        MatchFinders!();
    };
}

macro_rules! LzEncoder {
    () => {
        deps!();
        pub (crate) struct LzEncoder { pub (crate) data : LzEncoderData , pub (crate) matches : Matches , pub (crate) match_finder : MatchFinders , }
    };
}

LzEncoder!();