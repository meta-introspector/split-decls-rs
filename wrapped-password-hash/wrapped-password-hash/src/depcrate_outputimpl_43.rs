// Generated macro for impl_43 (impl)
macro_rules! Depcrate_outputimpl_43 {
() => {
// Module: crate::output
// Provides: {"impl_43"}
// Dependencies: {}
impl fmt :: Display for Output { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut buffer = [0u8 ; Self :: B64_MAX_LENGTH] ; self . encode (& mut buffer , self . encoding) . map_err (| _ | fmt :: Error) . and_then (| encoded | f . write_str (encoded)) } }
};
}
