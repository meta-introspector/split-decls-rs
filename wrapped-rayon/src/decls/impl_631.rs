macro_rules! deps {
    () => {
        InterleaveProducer!();
        Producer!();
    };
}

macro_rules! impl_631 {
    () => {
        deps!();
        impl < I , J > InterleaveProducer < I , J > where I : Producer , J : Producer < Item = I :: Item > , { fn new (i : I , j : J , i_len : usize , j_len : usize , i_next : bool) -> InterleaveProducer < I , J > { InterleaveProducer { i , j , i_len , j_len , i_next , } } }
    };
}

impl_631!()