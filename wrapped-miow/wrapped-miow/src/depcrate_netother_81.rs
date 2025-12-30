// Generated macro for other_81 (other)
macro_rules! Depcrate_netother_81 {
() => {
// Module: crate::net
// Provides: {"other_81"}
// Dependencies: {}
# [doc = " A type with the same memory layout as `SOCKADDR`. Used in converting Rust level"] # [doc = " SocketAddr* types into their system representation. The benefit of this specific"] # [doc = " type over using `SOCKADDR_STORAGE` is that this type is exactly as large as it"] # [doc = " needs to be and not a lot larger. And it can be initialized cleaner from Rust."] # [repr (C)] pub (crate) union SocketAddrCRepr { v4 : SOCKADDR_IN , v6 : SOCKADDR_IN6 , }
};
}
