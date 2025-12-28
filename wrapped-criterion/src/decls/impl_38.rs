macro_rules! deps {
    () => {
        AsyncExecutor!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [cfg (feature = "async_tokio")] impl AsyncExecutor for tokio :: runtime :: Handle { fn block_on < T > (& self , future : impl Future < Output = T >) -> T { self . block_on (future) } }
    };
}

impl_38!()