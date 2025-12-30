// Generated macro for impl_263 (impl)
macro_rules! Depcrate_ioimpl_263 {
() => {
// Module: crate::io
// Provides: {"impl_263"}
// Dependencies: {}
impl < R : fmt :: Debug > fmt :: Debug for BufReader < R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BufReader") . field ("reader" , & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . cap - self . pos , self . buf . len ()) ,) . finish () } }
};
}
