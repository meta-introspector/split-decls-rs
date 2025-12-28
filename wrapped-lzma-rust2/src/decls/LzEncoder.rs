macro_rules! deps {
    () => {
        LzEncoderData!();
        MatchFinders!();
        Matches!();
    };
}

macro_rules! LzEncoder {
    () => {
        deps!();
        pub (crate) struct LzEncoder { pub (crate) data : LzEncoderData , pub (crate) matches : Matches , pub (crate) match_finder : MatchFinders , }
    };
}

LzEncoder!()