// Generated macro for wrap_key (function)
macro_rules! Depcrate_pkcs8wrap_key {
() => {
// Module: crate::pkcs8
// Provides: {"wrap_key"}
// Dependencies: {}
pub (crate) fn wrap_key (template : & Template , private_key : & [u8] , public_key : & [u8]) -> Document { let mut result = Document { bytes : [0 ; ec :: PKCS8_DOCUMENT_MAX_LEN] , len : template . bytes . len () + private_key . len () + public_key . len () , } ; wrap_key_ (template , private_key , public_key , & mut result . bytes [.. result . len] ,) ; result }
};
}
