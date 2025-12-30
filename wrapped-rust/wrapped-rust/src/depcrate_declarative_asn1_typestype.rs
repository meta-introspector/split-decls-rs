// Generated macro for Type (enum)
macro_rules! Depcrate_declarative_asn1_typesType {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"Type"}
// Dependencies: {}
# [doc = " Internal type representation for mapping between"] # [doc = " Python and ASN.1."] # [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.asn1")] pub enum Type { # [doc = " SEQUENCE (`class`, `dict`)"] # [doc = " The first element is the Python class that represents the sequence,"] # [doc = " the second element is a dict of the (already converted) fields of the class."] Sequence (pyo3 :: Py < pyo3 :: types :: PyType > , pyo3 :: Py < pyo3 :: types :: PyDict >) , # [doc = " SEQUENCEOF (`list[`T`]`)"] SequenceOf (pyo3 :: Py < AnnotatedType >) , # [doc = " OPTIONAL (`T | None`)"] Option (pyo3 :: Py < AnnotatedType >) , # [doc = " `bool` -> `Boolean`"] PyBool () , # [doc = " `int` -> `Integer`"] PyInt () , # [doc = " `bytes` -> `Octet String`"] PyBytes () , # [doc = " `str` -> `UTF8String`"] PyStr () , # [doc = " PrintableString (`str`)"] PrintableString () , # [doc = " UtcTime (`datetime`)"] UtcTime () , # [doc = " GeneralizedTime (`datetime`)"] GeneralizedTime () , }
};
}
