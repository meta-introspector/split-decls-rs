// Generated macro for impl_141 (impl)
macro_rules! Depcrate_requestimpl_141 {
() => {
// Module: crate::request
// Provides: {"impl_141"}
// Dependencies: {}
impl Debug for Request { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { f . debug_struct ("Request") . field ("query" , & self . query) . field ("operation_name" , & self . operation_name) . field ("variables" , & self . variables) . field ("extensions" , & self . extensions) . finish () } }
};
}
