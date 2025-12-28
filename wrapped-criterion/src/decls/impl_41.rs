macro_rules! deps {
    () => {
        AsyncStdExecutor!();
        AsyncExecutor!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        # [cfg (feature = "async_std")] impl AsyncExecutor for AsyncStdExecutor { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { async_std :: task :: block_on (future) } }
    };
}

impl_41!();