// Generated macro for impl_856 (impl)
macro_rules! Depcrate_serverimpl_856 {
() => {
// Module: crate::server
// Provides: {"impl_856"}
// Dependencies: {}
impl < T , B > fmt :: Debug for Connection < T , B > where T : fmt :: Debug , B : fmt :: Debug + Buf , { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("Connection") . field ("connection" , & self . connection) . finish () } }
};
}
