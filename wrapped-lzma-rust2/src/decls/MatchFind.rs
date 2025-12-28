macro_rules! deps {
    () => {
        LzEncoderData!();
        Matches!();
    };
}

macro_rules! MatchFind {
    () => {
        deps!();
        pub (crate) trait MatchFind { fn find_matches (& mut self , encoder : & mut LzEncoderData , matches : & mut Matches) ; fn skip (& mut self , encoder : & mut LzEncoderData , len : usize) ; }
    };
}

MatchFind!();