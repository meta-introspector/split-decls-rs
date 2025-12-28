macro_rules! deps {
    () => {
        IntersperseConsumer!();
        Consumer!();
    };
}

macro_rules! impl_655 {
    () => {
        deps!();
        impl < C , T > IntersperseConsumer < C , T > where C : Consumer < T > , { fn new (base : C , item : T) -> Self { IntersperseConsumer { base , item , clone_first : false . into () , } } }
    };
}

impl_655!();