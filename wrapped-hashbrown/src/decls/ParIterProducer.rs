macro_rules! deps {
    () => {
        RawIterRange!();
    };
}

macro_rules! ParIterProducer {
    () => {
        deps!();
        # [doc = " Producer which returns a `Bucket<T>` for every element."] struct ParIterProducer < T > { iter : RawIterRange < T > , }
    };
}

ParIterProducer!()