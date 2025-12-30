// Generated macro for Map (struct)
macro_rules! Depcrate_concurrent_stream_mapMap {
() => {
// Module: crate::concurrent_stream::map
// Provides: {"Map"}
// Dependencies: {}
# [doc = " Convert items from one type into another"] # [derive (Debug)] pub struct Map < CS , F , FutT , T , FutB , B > where CS : ConcurrentStream < Item = T , Future = FutT > , F : Fn (T) -> FutB , F : Clone , FutT : Future < Output = T > , FutB : Future < Output = B > , { inner : CS , f : F , _phantom : PhantomData < (FutT , T , FutB , B) > , }
};
}
