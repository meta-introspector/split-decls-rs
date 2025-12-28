macro_rules! deps {
    () => {
        UnindexedProducer!();
        UnindexedConsumer!();
        Splitter!();
    };
}

macro_rules! bridge_unindexed {
    () => {
        deps!();
        # [doc = " A variant of [`bridge_producer_consumer()`] where the producer is an unindexed producer."] pub fn bridge_unindexed < P , C > (producer : P , consumer : C) -> C :: Result where P : UnindexedProducer , C : UnindexedConsumer < P :: Item > , { let splitter = Splitter :: new () ; bridge_unindexed_producer_consumer (false , splitter , producer , consumer) }
    };
}

bridge_unindexed!()