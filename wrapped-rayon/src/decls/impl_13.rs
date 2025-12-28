macro_rules! deps {
    () => {
        Fissile!();
        SplitProducer!();
        SplitInclusiveProducer!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'p , P , V > SplitInclusiveProducer < 'p , P , V > where V : Fissile < P > + Send , { pub (super) fn new_incl (data : V , separator : & 'p P) -> Self { SplitProducer { tail : data . length () , data , separator , } } }
    };
}

impl_13!();