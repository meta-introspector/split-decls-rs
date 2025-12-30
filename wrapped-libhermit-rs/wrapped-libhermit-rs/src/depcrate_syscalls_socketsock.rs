// Generated macro for Sock (enum)
macro_rules! Depcrate_syscalls_socketSock {
() => {
// Module: crate::syscalls::socket
// Provides: {"Sock"}
// Dependencies: {}
# [derive (TryFromPrimitive , IntoPrimitive , PartialEq , Eq , Clone , Copy , Debug)] # [repr (u8)] pub enum Sock { Stream = 1 , Dgram = 2 , Raw = 3 , Seqpacket = 5 , }
};
}
