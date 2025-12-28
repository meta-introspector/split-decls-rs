macro_rules! deps {
    () => {
        Folder!();
        NoopConsumer!();
    };
}

macro_rules! impl_726 {
    () => {
        deps!();
        impl < T > Folder < T > for NoopConsumer { type Result = () ; fn consume (self , _item : T) -> Self { self } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { iter . into_iter () . for_each (drop) ; self } fn complete (self) { } fn full (& self) -> bool { false } }
    };
}

impl_726!();