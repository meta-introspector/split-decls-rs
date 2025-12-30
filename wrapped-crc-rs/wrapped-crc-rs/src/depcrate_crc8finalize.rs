// Generated macro for finalize (function)
macro_rules! Depcrate_crc8finalize {
() => {
// Module: crate::crc8
// Provides: {"finalize"}
// Dependencies: {}
const fn finalize (algorithm : & Algorithm < u8 > , mut crc : u8) -> u8 { if algorithm . refin ^ algorithm . refout { crc = crc . reverse_bits () ; } if ! algorithm . refout { crc >>= 8u8 - algorithm . width ; } crc ^ algorithm . xorout }
};
}
