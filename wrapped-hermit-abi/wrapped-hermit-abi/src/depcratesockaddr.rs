// Generated macro for sockaddr (struct)
macro_rules! Depcratesockaddr {
() => {
// Module: crate
// Provides: {"sockaddr"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone , Default)] pub struct sockaddr { pub sa_len : u8 , pub sa_family : sa_family_t , pub sa_data : [c_char ; 14] , }
};
}
