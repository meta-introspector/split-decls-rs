// Generated macro for impl_54 (impl)
macro_rules! Depcrate_spinwaitimpl_54 {
() => {
// Module: crate::spinwait
// Provides: {"impl_54"}
// Dependencies: {}
impl SpinWait { # [doc = " Creates a new `SpinWait`."] # [inline] pub fn new () -> Self { Self :: default () } # [doc = " Resets a `SpinWait` to its initial state."] # [inline] pub fn reset (& mut self) { self . counter = 0 ; } # [doc = " Spins until the sleep threshold has been reached."] # [doc = ""] # [doc = " This function returns whether the sleep threshold has been reached, at"] # [doc = " which point further spinning has diminishing returns and the thread"] # [doc = " should be parked instead."] # [doc = ""] # [doc = " The spin strategy will initially use a CPU-bound loop but will fall back"] # [doc = " to yielding the CPU to the OS after a few iterations."] # [inline] pub fn spin (& mut self) -> bool { if self . counter >= 10 { return false ; } self . counter += 1 ; if self . counter <= 3 { cpu_relax (1 << self . counter) ; } else { thread_parker :: thread_yield () ; } true } # [doc = " Spins without yielding the thread to the OS."] # [doc = ""] # [doc = " Instead, the backoff is simply capped at a maximum value. This can be"] # [doc = " used to improve throughput in `compare_exchange` loops that have high"] # [doc = " contention."] # [inline] pub fn spin_no_yield (& mut self) { self . counter += 1 ; if self . counter > 10 { self . counter = 10 ; } cpu_relax (1 << self . counter) ; } }
};
}
