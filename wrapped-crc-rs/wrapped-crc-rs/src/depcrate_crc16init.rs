// Generated macro for init (function)
macro_rules! Depcrate_crc16init {
() => {
// Module: crate::crc16
// Provides: {"init"}
// Dependencies: {}
const fn init (algorithm : & Algorithm < u16 > , initial : u16) -> u16 { if algorithm . refin { initial . reverse_bits () >> (16u8 - algorithm . width) } else { initial << (16u8 - algorithm . width) } }
};
}
