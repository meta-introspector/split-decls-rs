// Generated macro for python_class_to_annotated (function)
macro_rules! Depcrate_declarative_asn1_typespython_class_to_annotated {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"python_class_to_annotated"}
// Dependencies: {}
pub (crate) fn python_class_to_annotated < 'p > (py : pyo3 :: Python < 'p > , class : & pyo3 :: Bound < 'p , pyo3 :: types :: PyType > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , AnnotatedType > > { if let Ok (root) = class . getattr ("__asn1_root__") { Ok (root . cast_into :: < AnnotatedType > () ?) } else { pyo3 :: Bound :: new (py , non_root_type_to_annotated (py , class) ?) } }
};
}
