macro_rules! ReduceConsumer {
    () => {
        struct ReduceConsumer < 'r , R , ID > { identity : & 'r ID , reduce_op : & 'r R , }
    };
}

ReduceConsumer!();