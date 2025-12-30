// Generated macro for ForEachConsumer (struct)
macro_rules! Depcrate_concurrent_stream_for_eachForEachConsumer {
() => {
// Module: crate::concurrent_stream::for_each
// Provides: {"ForEachConsumer"}
// Dependencies: {}
# [pin_project] pub (crate) struct ForEachConsumer < FutT , T , F , FutB > where FutT : Future < Output = T > , F : Fn (T) -> FutB , FutB : Future < Output = () > , { count : Arc < AtomicUsize > , # [pin] group : FuturesUnordered < ForEachFut < F , FutT , T , FutB > > , limit : usize , f : F , _phantom : PhantomData < (T , FutB) > , }
};
}
