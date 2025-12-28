macro_rules! FilterMapConsumer {
    () => {
        struct FilterMapConsumer < 'p , C , P > { base : C , filter_op : & 'p P , }
    };
}

FilterMapConsumer!();