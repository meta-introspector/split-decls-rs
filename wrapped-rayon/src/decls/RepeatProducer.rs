macro_rules! deps {
    () => {
        Repeat!();
    };
}

macro_rules! RepeatProducer {
    () => {
        deps!();
        # [doc = " Unindexed producer for `Repeat`."] struct RepeatProducer < T : Clone + Send > { element : T , }
    };
}

RepeatProducer!()