// Generated macro for EnumerateFuture (struct)
macro_rules! Depcrate_concurrent_stream_enumerateEnumerateFuture {
() => {
// Module: crate::concurrent_stream::enumerate
// Provides: {"EnumerateFuture"}
// Dependencies: {}
# [doc = " Takes a future and maps it to another future via a closure"] # [derive (Debug)] # [pin_project :: pin_project] pub struct EnumerateFuture < FutT , T > where FutT : Future < Output = T > , { done : bool , # [pin] fut_t : FutT , count : usize , }
};
}
