// Generated macro for Segment (enum)
macro_rules! Depcrate_macosSegment {
() => {
// Module: crate::macos
// Provides: {"Segment"}
// Dependencies: {}
# [doc = " A Mach-O segment."] pub enum Segment < 'a > { # [doc = " A 32-bit Mach-O segment."] Segment32 (& 'a libc :: segment_command) , # [doc = " A 64-bit Mach-O segment."] Segment64 (& 'a libc :: segment_command_64) , }
};
}
