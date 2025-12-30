// Generated macro for impl_538 (impl)
macro_rules! Depcrate_dataloaderimpl_538 {
() => {
// Module: crate::dataloader
// Provides: {"impl_538"}
// Dependencies: {}
impl < T > DataLoader < T , NoCache > { # [doc = " Use `Loader` to create a [DataLoader] that does not cache records."] pub fn new < S , R > (loader : T , spawner : S) -> Self where S : Fn (BoxFuture < 'static , () >) -> R + Send + Sync + 'static , { Self { inner : Arc :: new (DataLoaderInner { requests : Mutex :: new (Default :: default ()) , loader , }) , cache_factory : NoCache , delay : Duration :: from_millis (1) , max_batch_size : 1000 , disable_cache : false . into () , spawner : Box :: new (move | fut | { spawner (fut) ; }) , } } }
};
}
