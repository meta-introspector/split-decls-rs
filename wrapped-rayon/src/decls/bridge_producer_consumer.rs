macro_rules! deps {
    () => {
        Consumer!();
        Producer!();
        LengthSplitter!();
    };
}

macro_rules! bridge_producer_consumer {
    () => {
        deps!();
        # [doc = " This helper function is used to \"connect\" a producer and a"] # [doc = " consumer. You may prefer to call [`bridge()`], which wraps this"] # [doc = " function. This function will draw items from `producer` and feed"] # [doc = " them to `consumer`, splitting and creating parallel tasks when"] # [doc = " needed."] # [doc = ""] # [doc = " This is useful when you are implementing your own parallel"] # [doc = " iterators: it is often used as the definition of the"] # [doc = " [`drive_unindexed`] or [`drive`] methods."] # [doc = ""] # [doc = " [`drive_unindexed`]: super::ParallelIterator::drive_unindexed()"] # [doc = " [`drive`]: super::IndexedParallelIterator::drive()"] pub fn bridge_producer_consumer < P , C > (len : usize , producer : P , consumer : C) -> C :: Result where P : Producer , C : Consumer < P :: Item > , { let splitter = LengthSplitter :: new (producer . min_len () , producer . max_len () , len) ; return helper (len , false , splitter , producer , consumer) ; fn helper < P , C > (len : usize , migrated : bool , mut splitter : LengthSplitter , producer : P , consumer : C ,) -> C :: Result where P : Producer , C : Consumer < P :: Item > , { if consumer . full () { consumer . into_folder () . complete () } else if splitter . try_split (len , migrated) { let mid = len / 2 ; let (left_producer , right_producer) = producer . split_at (mid) ; let (left_consumer , right_consumer , reducer) = consumer . split_at (mid) ; let (left_result , right_result) = join_context (| context | { helper (mid , context . migrated () , splitter , left_producer , left_consumer ,) } , | context | { helper (len - mid , context . migrated () , splitter , right_producer , right_consumer ,) } ,) ; reducer . reduce (left_result , right_result) } else { producer . fold_with (consumer . into_folder ()) . complete () } } }
    };
}

bridge_producer_consumer!();