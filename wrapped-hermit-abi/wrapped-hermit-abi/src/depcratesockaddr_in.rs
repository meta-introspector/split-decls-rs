// Generated macro for sockaddr_in (struct)
macro_rules! Depcratesockaddr_in {
() => {
// Module: crate
// Provides: {"sockaddr_in"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone , Default)] pub struct sockaddr_in { pub sin_len : u8 , pub sin_family : sa_family_t , pub sin_port : in_port_t , pub sin_addr : in_addr , pub sin_zero : [c_char ; 8] , }
};
}
