// Generated macro for Thread (struct)
macro_rules! Depcrate_rt_threadThread {
() => {
// Module: crate::rt::thread
// Provides: {"Thread"}
// Dependencies: {}
pub (crate) struct Thread { pub id : Id , # [doc = " If the thread is runnable, blocked, or terminated."] pub state : State , # [doc = " True if the thread is in a critical section"] pub critical : bool , # [doc = " The operation the thread is about to take"] pub (super) operation : Option < Operation > , # [doc = " Tracks observed causality"] pub causality : VersionVec , # [doc = " Tracks the view of the lastest release fence"] pub released : VersionVec , # [doc = " Tracks DPOR relations"] pub dpor_vv : VersionVec , # [doc = " Version at which the thread last yielded"] pub last_yield : Option < u16 > , # [doc = " Number of times the thread yielded"] pub yield_count : usize , locals : LocalMap , # [doc = " `tracing` span used to associate diagnostics with the current thread."] span : tracing :: Span , }
};
}
