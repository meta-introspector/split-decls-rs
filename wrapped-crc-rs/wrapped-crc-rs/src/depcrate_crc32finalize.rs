// Generated macro for finalize (function)
macro_rules! Depcrate_crc32finalize {
() => {
// Module: crate::crc32
// Provides: {"finalize"}
// Dependencies: {}
const fn finalize (algorithm : & Algorithm < u32 > , mut crc : u32) -> u32 { if algorithm . refin ^ algorithm . refout { crc = crc . reverse_bits () ; } if ! algorithm . refout { crc >>= 32u8 - algorithm . width ; } crc ^ algorithm . xorout }
};
}
