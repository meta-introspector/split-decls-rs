// Generated macro for impl_221 (impl)
macro_rules! Depcrate_multiimpl_221 {
() => {
// Module: crate::multi
// Provides: {"impl_221"}
// Dependencies: {}
impl fmt :: Debug for Events { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Events") . field ("input" , & (self . bits & curl_sys :: CURL_CSELECT_IN != 0)) . field ("output" , & (self . bits & curl_sys :: CURL_CSELECT_OUT != 0)) . field ("error" , & (self . bits & curl_sys :: CURL_CSELECT_ERR != 0)) . finish () } }
};
}
