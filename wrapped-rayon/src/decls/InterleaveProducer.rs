macro_rules! deps {
    () => {
        Producer!();
    };
}

macro_rules! InterleaveProducer {
    () => {
        deps!();
        struct InterleaveProducer < I , J > where I : Producer , J : Producer < Item = I :: Item > , { i : I , j : J , i_len : usize , j_len : usize , i_next : bool , }
    };
}

InterleaveProducer!();