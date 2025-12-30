// Generated macro for emit_flight_on_path (function)
macro_rules! Depcrate_test_utilsemit_flight_on_path {
() => {
// Module: crate::test_utils
// Provides: {"emit_flight_on_path"}
// Dependencies: {}
pub fn emit_flight_on_path (conn : & mut Connection , from : Option < SocketAddr > , to : Option < SocketAddr > ,) -> Result < Vec < (Vec < u8 > , SendInfo) > > { emit_flight_with_max_buffer (conn , 65535 , from , to) }
};
}
