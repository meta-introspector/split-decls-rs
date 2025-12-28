macro_rules! WorkPoolConfig {
    () => {
        # [doc = " Configuration for a work pool."] # [derive (Debug , Clone)] pub (crate) struct WorkPoolConfig { pub (crate) num_workers : u32 , pub (crate) num_work : u64 , }
    };
}

WorkPoolConfig!();