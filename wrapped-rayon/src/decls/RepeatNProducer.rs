macro_rules! deps {
    () => {
        RepeatN!();
        Empty!();
        Producer!();
    };
}

macro_rules! RepeatNProducer {
    () => {
        deps!();
        # [doc = " Producer for `RepeatN`."] # [derive (Clone)] enum RepeatNProducer < T > { Repeats (T , NonZeroUsize) , Empty , }
    };
}

RepeatNProducer!()