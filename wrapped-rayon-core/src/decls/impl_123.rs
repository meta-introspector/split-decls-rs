macro_rules! deps {
    () => {
        WorkerThread!();
        ThreadBuilder!();
        JobFifo!();
        XorShift64Star!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl From < ThreadBuilder > for WorkerThread { fn from (thread : ThreadBuilder) -> Self { Self { worker : thread . worker , stealer : thread . stealer , fifo : JobFifo :: new () , index : thread . index , rng : XorShift64Star :: new () , registry : thread . registry , } } }
    };
}

impl_123!();