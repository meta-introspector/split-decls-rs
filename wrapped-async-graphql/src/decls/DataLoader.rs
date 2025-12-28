macro_rules! deps {
    () => {
        NoCache!();
        DataLoaderInner!();
        Data!();
    };
}

macro_rules! DataLoader {
    () => {
        deps!();
        # [doc = " Data loader."] # [doc = ""] # [doc = " Reference: <https://github.com/facebook/dataloader>"] pub struct DataLoader < T , C = NoCache > { inner : Arc < DataLoaderInner < T > > , cache_factory : C , delay : Duration , max_batch_size : usize , disable_cache : AtomicBool , spawner : Box < dyn Fn (BoxFuture < 'static , () >) + Send + Sync > , }
    };
}

DataLoader!();