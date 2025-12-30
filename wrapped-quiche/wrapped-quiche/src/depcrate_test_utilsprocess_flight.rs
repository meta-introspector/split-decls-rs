// Generated macro for process_flight (function)
macro_rules! Depcrate_test_utilsprocess_flight {
() => {
// Module: crate::test_utils
// Provides: {"process_flight"}
// Dependencies: {}
pub fn process_flight (conn : & mut Connection , flight : Vec < (Vec < u8 > , SendInfo) > ,) -> Result < () > { for (mut pkt , si) in flight { let info = RecvInfo { to : si . to , from : si . from , } ; conn . recv (& mut pkt , info) ? ; } Ok (()) }
};
}
