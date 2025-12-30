// Generated macro for finalize (function)
macro_rules! Depcrate_crc128finalize {
() => {
// Module: crate::crc128
// Provides: {"finalize"}
// Dependencies: {}
const fn finalize (algorithm : & Algorithm < u128 > , mut crc : u128) -> u128 { if algorithm . refin ^ algorithm . refout { crc = crc . reverse_bits () ; } if ! algorithm . refout { crc >>= 128u8 - algorithm . width ; } crc ^ algorithm . xorout }
};
}
