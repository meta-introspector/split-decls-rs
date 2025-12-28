macro_rules! deps {
    () => {
        IntersperseProducer!();
        Producer!();
    };
}

macro_rules! impl_648 {
    () => {
        deps!();
        impl < P > IntersperseProducer < P > where P : Producer , { fn new (base : P , item : P :: Item , len : usize) -> Self { IntersperseProducer { base , item , len , clone_first : false , } } }
    };
}

impl_648!()