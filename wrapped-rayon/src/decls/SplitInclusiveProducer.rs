macro_rules! deps {
    () => {
        SplitProducer!();
    };
}

macro_rules! SplitInclusiveProducer {
    () => {
        deps!();
        pub (super) type SplitInclusiveProducer < 'p , P , V > = SplitProducer < 'p , P , V , true > ;
    };
}

SplitInclusiveProducer!();