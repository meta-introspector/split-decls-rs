// Generated macro for impl_131 (impl)
macro_rules! Depcrate_addressimpl_131 {
() => {
// Module: crate::address
// Provides: {"impl_131"}
// Dependencies: {}
impl < M > PartialEq for Recipient < M > where M : Message + Send , M :: Result : Send , { fn eq (& self , other : & Self) -> bool { self . tx . hash () == other . tx . hash () } }
};
}
