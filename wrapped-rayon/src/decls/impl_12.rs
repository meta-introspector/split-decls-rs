macro_rules! deps {
    () => {
        Fissile!();
        SplitProducer!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'p , P , V > SplitProducer < 'p , P , V > where V : Fissile < P > + Send , { pub (super) fn new (data : V , separator : & 'p P) -> Self { SplitProducer { tail : data . length () , data , separator , } } }
    };
}

impl_12!();