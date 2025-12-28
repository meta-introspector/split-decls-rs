macro_rules! deps {
    () => {
        ChainProducer!();
        Producer!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl < A , B > ChainProducer < A , B > where A : Producer , B : Producer < Item = A :: Item > , { fn new (a_len : usize , a : A , b : B) -> Self { ChainProducer { a_len , a , b } } }
    };
}

impl_307!();