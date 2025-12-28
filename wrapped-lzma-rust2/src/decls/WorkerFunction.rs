macro_rules! deps {
    () => {
        WorkerHandle!();
        Error!();
    };
}

macro_rules! WorkerFunction {
    () => {
        deps!();
        pub (crate) type WorkerFunction < W , R > = fn (WorkerHandle < (u64 , W) > , SyncSender < (u64 , R) > , Arc < AtomicBool > , Arc < Mutex < Option < io :: Error > > > , Arc < AtomicU32 > ,) ;
    };
}

WorkerFunction!()