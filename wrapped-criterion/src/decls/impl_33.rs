macro_rules! deps {
    () => {
        AsyncExecutor!();
        FuturesExecutor!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [cfg (feature = "async_futures")] impl AsyncExecutor for FuturesExecutor { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { futures :: executor :: block_on (future) } }
    };
}

impl_33!();