// Generated macro for probes_from_flags (function)
macro_rules! Depcrate_deflate_coreprobes_from_flags {
() => {
// Module: crate::deflate::core
// Provides: {"probes_from_flags"}
// Dependencies: {}
const fn probes_from_flags (flags : u32) -> [u32 ; 2] { [1 + ((flags & 0xFFF) + 2) / 3 , 1 + (((flags & 0xFFF) >> 2) + 2) / 3 ,] }
};
}
