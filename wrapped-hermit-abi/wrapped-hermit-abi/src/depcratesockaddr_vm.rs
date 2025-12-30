// Generated macro for sockaddr_vm (struct)
macro_rules! Depcratesockaddr_vm {
() => {
// Module: crate
// Provides: {"sockaddr_vm"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone , Default)] pub struct sockaddr_vm { pub svm_len : u8 , pub svm_family : sa_family_t , pub svm_reserved1 : u16 , pub svm_port : u32 , pub svm_cid : u32 , pub svm_zero : [u8 ; 4] , }
};
}
