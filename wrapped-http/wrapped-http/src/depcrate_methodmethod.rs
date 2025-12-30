// Generated macro for Method (struct)
macro_rules! Depcrate_methodMethod {
() => {
// Module: crate::method
// Provides: {"Method"}
// Dependencies: {}
# [doc = " The Request Method (VERB)"] # [doc = ""] # [doc = " This type also contains constants for a number of common HTTP methods such"] # [doc = " as GET, POST, etc."] # [doc = ""] # [doc = " Currently includes 8 variants representing the 8 methods defined in"] # [doc = " [RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH,"] # [doc = " and an Extension variant for all extensions."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use http::Method;"] # [doc = ""] # [doc = " assert_eq!(Method::GET, Method::from_bytes(b\"GET\").unwrap());"] # [doc = " assert!(Method::GET.is_idempotent());"] # [doc = " assert_eq!(Method::POST.as_str(), \"POST\");"] # [doc = " ```"] # [derive (Clone , PartialEq , Eq , Hash)] pub struct Method (Inner) ;
};
}
