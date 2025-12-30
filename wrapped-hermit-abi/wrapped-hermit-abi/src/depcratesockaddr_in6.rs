// Generated macro for sockaddr_in6 (struct)
macro_rules! Depcratesockaddr_in6 {
() => {
// Module: crate
// Provides: {"sockaddr_in6"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone , Default)] pub struct sockaddr_in6 { pub sin6_len : u8 , pub sin6_family : sa_family_t , pub sin6_port : in_port_t , pub sin6_flowinfo : u32 , pub sin6_addr : in6_addr , pub sin6_scope_id : u32 , }
};
}
