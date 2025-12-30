// Generated macro for Barrier (struct)
macro_rules! Depcrate_barrierBarrier {
() => {
// Module: crate::barrier
// Provides: {"Barrier"}
// Dependencies: {}
# [doc = " A counter to synchronize multiple tasks at the same time."] # [derive (Debug)] pub struct Barrier { n : usize , state : Mutex < State > , event : Event , }
};
}
