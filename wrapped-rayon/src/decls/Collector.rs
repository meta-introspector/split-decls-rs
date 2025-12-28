macro_rules! deps {
    () => {
        ParallelExtend!();
        FromParallelIterator!();
    };
}

macro_rules! Collector {
    () => {
        deps!();
        # [doc = " Shim to implement a one-time `ParallelExtend` using `FromParallelIterator`."] struct Collector < FromT > { result : Option < FromT > , }
    };
}

Collector!();