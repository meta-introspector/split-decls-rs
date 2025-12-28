macro_rules! TryReduceWithConsumer {
    () => {
        struct TryReduceWithConsumer < 'r , R > { reduce_op : & 'r R , full : & 'r AtomicBool , }
    };
}

TryReduceWithConsumer!()