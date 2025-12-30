// Generated macro for emit_flight (function)
macro_rules! Depcrate_test_utilsemit_flight {
() => {
// Module: crate::test_utils
// Provides: {"emit_flight"}
// Dependencies: {}
pub fn emit_flight (conn : & mut Connection) -> Result < Vec < (Vec < u8 > , SendInfo) > > { emit_flight_on_path (conn , None , None) }
};
}
