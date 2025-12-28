macro_rules! deps {
    () => {
        RawIterRange!();
    };
}

macro_rules! ParDrainProducer {
    () => {
        deps!();
        # [doc = " Producer which will consume all elements in the range, even if it is dropped"] # [doc = " halfway through."] struct ParDrainProducer < T > { iter : RawIterRange < T > , }
    };
}

ParDrainProducer!();