macro_rules! TryReduceFolder {
    () => {
        struct TryReduceFolder < 'r , R , T : Try > { reduce_op : & 'r R , control : ControlFlow < T :: Residual , T :: Output > , full : & 'r AtomicBool , }
    };
}

TryReduceFolder!()