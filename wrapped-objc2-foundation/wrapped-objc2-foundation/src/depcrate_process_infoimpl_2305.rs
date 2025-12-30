// Generated macro for impl_2305 (impl)
macro_rules! Depcrate_process_infoimpl_2305 {
() => {
// Module: crate::process_info
// Provides: {"impl_2305"}
// Dependencies: {}
impl fmt :: Debug for NSProcessInfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut debug = f . debug_struct ("NSProcessInfo") ; # [cfg (feature = "NSString")] debug . field ("processName" , & self . processName ()) ; debug . finish_non_exhaustive () } }
};
}
