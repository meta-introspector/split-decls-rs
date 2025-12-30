// Generated macro for sockaddr_storage (struct)
macro_rules! Depcratesockaddr_storage {
() => {
// Module: crate
// Provides: {"sockaddr_storage"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct sockaddr_storage { pub s2_len : u8 , pub ss_family : sa_family_t , __ss_pad1 : [u8 ; 6] , __ss_align : i64 , __ss_pad2 : [u8 ; 112] , }
};
}
