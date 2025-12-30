// Generated macro for init (function)
macro_rules! Depcrate_crc8init {
() => {
// Module: crate::crc8
// Provides: {"init"}
// Dependencies: {}
const fn init (algorithm : & Algorithm < u8 > , initial : u8) -> u8 { if algorithm . refin { initial . reverse_bits () >> (8u8 - algorithm . width) } else { initial << (8u8 - algorithm . width) } }
};
}
