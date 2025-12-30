// Generated macro for impl_2151 (impl)
macro_rules! Depcrate_io_buf_readerimpl_2151 {
() => {
// Module: crate::io::buf_reader
// Provides: {"impl_2151"}
// Dependencies: {}
impl < R : fmt :: Debug > fmt :: Debug for BufReader < R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BufReader") . field ("reader" , & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . cap - self . pos , self . buffer . len ())) . finish () } }
};
}
