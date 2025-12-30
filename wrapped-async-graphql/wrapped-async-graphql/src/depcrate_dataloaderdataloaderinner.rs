// Generated macro for DataLoaderInner (struct)
macro_rules! Depcrate_dataloaderDataLoaderInner {
() => {
// Module: crate::dataloader
// Provides: {"DataLoaderInner"}
// Dependencies: {}
struct DataLoaderInner < T > { requests : Mutex < FnvHashMap < TypeId , Box < dyn Any + Sync + Send > > > , loader : T , }
};
}
