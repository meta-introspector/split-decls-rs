macro_rules! deps {
    () => {
        SmolExecutor!();
        AsyncExecutor!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "async_smol")] impl AsyncExecutor for SmolExecutor { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { smol :: block_on (future) } }
    };
}

impl_35!()