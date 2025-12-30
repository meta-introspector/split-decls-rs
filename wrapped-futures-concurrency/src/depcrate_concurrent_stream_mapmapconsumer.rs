// Generated macro for MapConsumer (struct)
macro_rules! Depcrate_concurrent_stream_mapMapConsumer {
() => {
// Module: crate::concurrent_stream::map
// Provides: {"MapConsumer"}
// Dependencies: {}
# [pin_project] pub struct MapConsumer < C , F , FutT , T , FutB , B > where FutT : Future < Output = T > , C : Consumer < B , MapFuture < F , FutT , T , FutB , B > > , F : Fn (T) -> FutB , F : Clone , FutB : Future < Output = B > , { # [pin] inner : C , f : F , _phantom : PhantomData < (FutT , T , FutB , B) > , }
};
}
