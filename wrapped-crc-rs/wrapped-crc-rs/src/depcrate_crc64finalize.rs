// Generated macro for finalize (function)
macro_rules! Depcrate_crc64finalize {
() => {
// Module: crate::crc64
// Provides: {"finalize"}
// Dependencies: {}
const fn finalize (algorithm : & Algorithm < u64 > , mut crc : u64) -> u64 { if algorithm . refin ^ algorithm . refout { crc = crc . reverse_bits () ; } if ! algorithm . refout { crc >>= 64u8 - algorithm . width ; } crc ^ algorithm . xorout }
};
}
