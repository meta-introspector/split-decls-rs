// Generated macro for impl_2173 (impl)
macro_rules! Depcrate_io_buf_writerimpl_2173 {
() => {
// Module: crate::io::buf_writer
// Provides: {"impl_2173"}
// Dependencies: {}
impl < W : fmt :: Debug > fmt :: Debug for BufWriter < W > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BufWriter") . field ("writer" , & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . buf . len () , self . buf . capacity ())) . field ("written" , & self . written) . finish () } }
};
}
