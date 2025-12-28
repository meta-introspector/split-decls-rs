macro_rules! ReduceFolder {
    () => {
        struct ReduceFolder < 'r , R , T > { reduce_op : & 'r R , item : T , }
    };
}

ReduceFolder!();