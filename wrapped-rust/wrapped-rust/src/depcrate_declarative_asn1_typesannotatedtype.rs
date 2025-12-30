// Generated macro for AnnotatedType (struct)
macro_rules! Depcrate_declarative_asn1_typesAnnotatedType {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"AnnotatedType"}
// Dependencies: {}
# [doc = " A type that we know how to encode/decode, along with any"] # [doc = " annotations that influence encoding/decoding."] # [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.asn1")] # [derive (Debug)] pub struct AnnotatedType { pub inner : pyo3 :: Py < Type > , # [pyo3 (get)] pub annotation : pyo3 :: Py < Annotation > , }
};
}
