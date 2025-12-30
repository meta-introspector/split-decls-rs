// Generated macro for impl_472 (impl)
macro_rules! Depcrate_errorimpl_472 {
() => {
// Module: crate::error
// Provides: {"impl_472"}
// Dependencies: {}
impl fmt :: Display for CryptographyError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { CryptographyError :: Asn1Parse (asn1_error) => { write ! (f , "error parsing asn1 value: {asn1_error:?}") } CryptographyError :: Asn1Write (asn1 :: WriteError :: AllocationError) => { write ! (f , "failed to allocate memory while performing ASN.1 serialization") } CryptographyError :: KeyParsing (asn1_error) => { write ! (f , "Could not deserialize key data. The data may be in an incorrect format, it may be encrypted with an unsupported algorithm, or it may be an unsupported key type (e.g. EC curves with explicit parameters). Details: {asn1_error}" ,) } CryptographyError :: Py (py_error) => write ! (f , "{py_error}") , CryptographyError :: OpenSSL (error_stack) => { write ! (f , "Unknown OpenSSL error. This error is commonly encountered
                    when another library is not cleaning up the OpenSSL error
                    stack. If you are using cryptography with another library
                    that uses OpenSSL try disabling it before reporting a bug.
                    Otherwise please file an issue at
                    https://github.com/pyca/cryptography/issues with
                    information on how to reproduce this. ({error_stack})") } } } }
};
}
