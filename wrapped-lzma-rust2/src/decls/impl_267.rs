macro_rules! deps {
    () => {
        WorkPoolConfig!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl WorkPoolConfig { pub (crate) fn new (num_workers : u32 , num_work : u64) -> Self { Self { num_workers , num_work , } } }
    };
}

impl_267!()