// Generated macro for init (function)
macro_rules! Depcrate_crc128init {
() => {
// Module: crate::crc128
// Provides: {"init"}
// Dependencies: {}
const fn init (algorithm : & Algorithm < u128 > , initial : u128) -> u128 { if algorithm . refin { initial . reverse_bits () >> (128u8 - algorithm . width) } else { initial << (128u8 - algorithm . width) } }
};
}
