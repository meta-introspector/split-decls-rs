// Generated macro for StatusCode (struct)
macro_rules! Depcrate_statusStatusCode {
() => {
// Module: crate::status
// Provides: {"StatusCode"}
// Dependencies: {}
# [doc = " An HTTP status code (`status-code` in RFC 9110 et al.)."] # [doc = ""] # [doc = " Constants are provided for known status codes, including those in the IANA"] # [doc = " [HTTP Status Code Registry]("] # [doc = " https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)."] # [doc = ""] # [doc = " Status code values in the range 100-999 (inclusive) are supported by this"] # [doc = " type. Values in the range 100-599 are semantically classified by the most"] # [doc = " significant digit. See [`StatusCode::is_success`], etc. Values above 599"] # [doc = " are unclassified but allowed for legacy compatibility, though their use is"] # [doc = " discouraged. Applications may interpret such values as protocol errors."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use http::StatusCode;"] # [doc = ""] # [doc = " assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);"] # [doc = " assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);"] # [doc = " assert!(StatusCode::OK.is_success());"] # [doc = " ```"] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct StatusCode (NonZeroU16) ;
};
}
