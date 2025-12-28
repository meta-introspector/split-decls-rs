macro_rules! deps {
    () => {
        LiteralSubCoder!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl LiteralSubCoder { pub fn new () -> Self { let probs = [PROB_INIT ; 0x300] ; Self { probs } } pub fn reset (& mut self) { self . probs = [PROB_INIT ; 0x300] ; } }
    };
}

impl_52!()