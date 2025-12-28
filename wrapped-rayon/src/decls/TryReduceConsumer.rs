macro_rules! TryReduceConsumer {
    () => {
        struct TryReduceConsumer < 'r , R , ID > { identity : & 'r ID , reduce_op : & 'r R , full : & 'r AtomicBool , }
    };
}

TryReduceConsumer!()