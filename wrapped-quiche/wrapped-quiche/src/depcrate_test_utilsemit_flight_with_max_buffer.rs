// Generated macro for emit_flight_with_max_buffer (function)
macro_rules! Depcrate_test_utilsemit_flight_with_max_buffer {
() => {
// Module: crate::test_utils
// Provides: {"emit_flight_with_max_buffer"}
// Dependencies: {}
pub fn emit_flight_with_max_buffer (conn : & mut Connection , out_size : usize , from : Option < SocketAddr > , to : Option < SocketAddr > ,) -> Result < Vec < (Vec < u8 > , SendInfo) > > { let mut flight = Vec :: new () ; loop { let mut out = vec ! [0u8 ; out_size] ; let info = match conn . send_on_path (& mut out , from , to) { Ok ((written , info)) => { out . truncate (written) ; info } , Err (Error :: Done) => break , Err (e) => return Err (e) , } ; flight . push ((out , info)) ; } if flight . is_empty () { return Err (Error :: Done) ; } Ok (flight) }
};
}
