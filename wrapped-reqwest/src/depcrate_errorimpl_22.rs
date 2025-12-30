// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut builder = f . debug_struct ("reqwest::Error") ; builder . field ("kind" , & self . inner . kind) ; if let Some (ref url) = self . inner . url { builder . field ("url" , & url . as_str ()) ; } if let Some (ref source) = self . inner . source { builder . field ("source" , source) ; } builder . finish () } }
};
}
