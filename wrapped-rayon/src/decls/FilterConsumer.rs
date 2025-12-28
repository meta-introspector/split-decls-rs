macro_rules! FilterConsumer {
    () => {
        struct FilterConsumer < 'p , C , P > { base : C , filter_op : & 'p P , }
    };
}

FilterConsumer!();