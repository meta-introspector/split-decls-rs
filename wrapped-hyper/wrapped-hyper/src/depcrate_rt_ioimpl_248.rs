// Generated macro for impl_248 (impl)
macro_rules! Depcrate_rt_ioimpl_248 {
() => {
// Module: crate::rt::io
// Provides: {"impl_248"}
// Dependencies: {}
impl fmt :: Debug for ReadBuf < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ReadBuf") . field ("filled" , & self . filled) . field ("init" , & self . init) . field ("capacity" , & self . capacity ()) . finish () } }
};
}
