macro_rules! deps {
    () => {
        Splitter!();
        UnindexedProducer!();
        UnindexedConsumer!();
    };
}

macro_rules! bridge_unindexed_producer_consumer {
    () => {
        deps!();
        fn bridge_unindexed_producer_consumer < P , C > (migrated : bool , mut splitter : Splitter , producer : P , consumer : C ,) -> C :: Result where P : UnindexedProducer , C : UnindexedConsumer < P :: Item > , { if consumer . full () { consumer . into_folder () . complete () } else if splitter . try_split (migrated) { match producer . split () { (left_producer , Some (right_producer)) => { let (reducer , left_consumer , right_consumer) = (consumer . to_reducer () , consumer . split_off_left () , consumer) ; let bridge = bridge_unindexed_producer_consumer ; let (left_result , right_result) = join_context (| context | bridge (context . migrated () , splitter , left_producer , left_consumer) , | context | bridge (context . migrated () , splitter , right_producer , right_consumer) ,) ; reducer . reduce (left_result , right_result) } (producer , None) => producer . fold_with (consumer . into_folder ()) . complete () , } } else { producer . fold_with (consumer . into_folder ()) . complete () } }
    };
}

bridge_unindexed_producer_consumer!()