// Generated macro for VTimestamp (struct)
macro_rules! Depcrate_concurrency_vector_clockVTimestamp {
() => {
// Module: crate::concurrency::vector_clock
// Provides: {"VTimestamp"}
// Dependencies: {}
# [doc = " The time-stamps recorded in the data-race detector consist of both"] # [doc = " a 32-bit unsigned integer which is the actual timestamp, and a `Span`"] # [doc = " so that diagnostics can report what code was responsible for an operation."] # [derive (Clone , Copy , Debug)] pub (super) struct VTimestamp { # [doc = " The lowest bit indicates read type, the rest is the time."] # [doc = " `1` indicates a retag read, `0` a regular read."] time_and_read_type : u32 , pub span : Span , }
};
}
