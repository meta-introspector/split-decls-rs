// Generated macro for DelayNs (trait)
macro_rules! Depcrate_delayDelayNs {
() => {
// Module: crate::delay
// Provides: {"DelayNs"}
// Dependencies: {}
# [doc = " Delay with up to nanosecond precision."] pub trait DelayNs { # [doc = " Pauses execution for at minimum `ns` nanoseconds. Pause can be longer"] # [doc = " if the implementation requires it due to precision/timing issues."] fn delay_ns (& mut self , ns : u32) ; # [doc = " Pauses execution for at minimum `us` microseconds. Pause can be longer"] # [doc = " if the implementation requires it due to precision/timing issues."] fn delay_us (& mut self , mut us : u32) { const MAX_MICROS : u32 = u32 :: MAX / NANOS_PER_MICRO ; while us > MAX_MICROS { us -= MAX_MICROS ; self . delay_ns (MAX_MICROS * NANOS_PER_MICRO) ; } self . delay_ns (us * NANOS_PER_MICRO) ; } # [doc = " Pauses execution for at minimum `ms` milliseconds. Pause can be longer"] # [doc = " if the implementation requires it due to precision/timing issues."] # [inline] fn delay_ms (& mut self , mut ms : u32) { const MAX_MILLIS : u32 = u32 :: MAX / NANOS_PER_MILLI ; while ms > MAX_MILLIS { ms -= MAX_MILLIS ; self . delay_ns (MAX_MILLIS * NANOS_PER_MILLI) ; } self . delay_ns (ms * NANOS_PER_MILLI) ; } }
};
}
