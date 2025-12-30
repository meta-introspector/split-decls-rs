// Generated macro for impl_344 (impl)
macro_rules! Depcrate_statusimpl_344 {
() => {
// Module: crate::status
// Provides: {"impl_344"}
// Dependencies: {}
# [doc = " Formats the status code, *including* the canonical reason."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use http::StatusCode;"] # [doc = " assert_eq!(format!(\"{}\", StatusCode::OK), \"200 OK\");"] # [doc = " ```"] impl fmt :: Display for StatusCode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} {}" , u16 :: from (* self) , self . canonical_reason () . unwrap_or ("<unknown status code>")) } }
};
}
