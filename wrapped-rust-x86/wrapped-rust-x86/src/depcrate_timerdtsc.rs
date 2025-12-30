// Generated macro for rdtsc (function)
macro_rules! Depcrate_timerdtsc {
() => {
// Module: crate::time
// Provides: {"rdtsc"}
// Dependencies: {}
# [doc = " Read the time stamp counter."] # [doc = ""] # [doc = " The RDTSC instruction is not a serializing instruction."] # [doc = " It does not necessarily wait until all previous instructions"] # [doc = " have been executed before reading the counter. Similarly,"] # [doc = " subsequent instructions may begin execution before the"] # [doc = " read operation is performed. If software requires RDTSC to be"] # [doc = " executed only after all previous instructions have completed locally,"] # [doc = " it can either use RDTSCP or execute the sequence LFENCE;RDTSC."] # [doc = ""] # [doc = " # Safety"] # [doc = " * Causes a GP fault if the TSD flag in register CR4 is set and the CPL"] # [doc = "   is greater than 0."] pub unsafe fn rdtsc () -> u64 { _rdtsc () as u64 }
};
}
