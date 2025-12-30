// Generated macro for DataLoader (struct)
macro_rules! Depcrate_dataloaderDataLoader {
() => {
// Module: crate::dataloader
// Provides: {"DataLoader"}
// Dependencies: {}
# [doc = " Data loader."] # [doc = ""] # [doc = " Reference: <https://github.com/facebook/dataloader>"] pub struct DataLoader < T , C = NoCache > { inner : Arc < DataLoaderInner < T > > , cache_factory : C , delay : Duration , max_batch_size : usize , disable_cache : AtomicBool , spawner : Box < dyn Fn (BoxFuture < 'static , () >) + Send + Sync > , }
};
}
