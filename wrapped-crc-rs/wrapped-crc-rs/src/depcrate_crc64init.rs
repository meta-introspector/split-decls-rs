// Generated macro for init (function)
macro_rules! Depcrate_crc64init {
() => {
// Module: crate::crc64
// Provides: {"init"}
// Dependencies: {}
const fn init (algorithm : & Algorithm < u64 > , initial : u64) -> u64 { if algorithm . refin { initial . reverse_bits () >> (64u8 - algorithm . width) } else { initial << (64u8 - algorithm . width) } }
};
}
