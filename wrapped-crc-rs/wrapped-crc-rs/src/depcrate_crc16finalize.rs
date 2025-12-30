// Generated macro for finalize (function)
macro_rules! Depcrate_crc16finalize {
() => {
// Module: crate::crc16
// Provides: {"finalize"}
// Dependencies: {}
const fn finalize (algorithm : & Algorithm < u16 > , mut crc : u16) -> u16 { if algorithm . refin ^ algorithm . refout { crc = crc . reverse_bits () ; } if ! algorithm . refout { crc >>= 16u8 - algorithm . width ; } crc ^ algorithm . xorout }
};
}
