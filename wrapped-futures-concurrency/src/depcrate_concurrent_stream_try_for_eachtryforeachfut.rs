// Generated macro for TryForEachFut (struct)
macro_rules! Depcrate_concurrent_stream_try_for_eachTryForEachFut {
() => {
// Module: crate::concurrent_stream::try_for_each
// Provides: {"TryForEachFut"}
// Dependencies: {}
# [doc = " Takes a future and maps it to another future via a closure"] # [derive (Debug)] pub struct TryForEachFut < F , FutT , T , FutB , B > where FutT : Future < Output = T > , F : Clone + Fn (T) -> FutB , FutB : Future < Output = B > , B : Try < Output = () > , { done : bool , count : Arc < AtomicUsize > , f : F , fut_t : Option < FutT > , fut_b : Option < FutB > , }
};
}
