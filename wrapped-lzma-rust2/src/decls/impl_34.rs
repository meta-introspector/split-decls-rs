macro_rules! deps {
    () => {
        MatchFind!();
        LzEncoderData!();
        MatchFinders!();
        Bt4!();
        Matches!();
        Hc4!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl MatchFind for MatchFinders { fn find_matches (& mut self , encoder : & mut LzEncoderData , matches : & mut Matches) { match self { MatchFinders :: Hc4 (m) => m . find_matches (encoder , matches) , MatchFinders :: Bt4 (m) => m . find_matches (encoder , matches) , } } fn skip (& mut self , encoder : & mut LzEncoderData , len : usize) { match self { MatchFinders :: Hc4 (m) => m . skip (encoder , len) , MatchFinders :: Bt4 (m) => m . skip (encoder , len) , } } }
    };
}

impl_34!();