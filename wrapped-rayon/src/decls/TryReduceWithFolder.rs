macro_rules! TryReduceWithFolder {
    () => {
        struct TryReduceWithFolder < 'r , R , T : Try > { reduce_op : & 'r R , opt_control : Option < ControlFlow < T :: Residual , T :: Output > > , full : & 'r AtomicBool , }
    };
}

TryReduceWithFolder!()