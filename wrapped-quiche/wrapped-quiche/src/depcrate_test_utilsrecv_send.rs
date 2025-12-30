// Generated macro for recv_send (function)
macro_rules! Depcrate_test_utilsrecv_send {
() => {
// Module: crate::test_utils
// Provides: {"recv_send"}
// Dependencies: {}
pub fn recv_send < F : BufFactory > (conn : & mut Connection < F > , buf : & mut [u8] , len : usize ,) -> Result < usize > { let active_path = conn . paths . get_active () ? ; let info = RecvInfo { to : active_path . local_addr () , from : active_path . peer_addr () , } ; conn . recv (& mut buf [.. len] , info) ? ; let mut off = 0 ; match conn . send (& mut buf [off ..]) { Ok ((write , _)) => off += write , Err (Error :: Done) => () , Err (e) => return Err (e) , } Ok (off) }
};
}
