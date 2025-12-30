// Generated macro for impl_516 (impl)
macro_rules! Depcrate_ffiimpl_516 {
() => {
// Module: crate::ffi
// Provides: {"impl_516"}
// Dependencies: {}
impl From < & RecvInfo < '_ > > for crate :: RecvInfo { fn from (info : & RecvInfo) -> crate :: RecvInfo { crate :: RecvInfo { from : std_addr_from_c (info . from , info . from_len) , to : std_addr_from_c (info . to , info . to_len) , } } }
};
}
