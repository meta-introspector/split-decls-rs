// Generated macro for init (function)
macro_rules! Depcrate_crc32init {
() => {
// Module: crate::crc32
// Provides: {"init"}
// Dependencies: {}
const fn init (algorithm : & Algorithm < u32 > , initial : u32) -> u32 { if algorithm . refin { initial . reverse_bits () >> (32u8 - algorithm . width) } else { initial << (32u8 - algorithm . width) } }
};
}
