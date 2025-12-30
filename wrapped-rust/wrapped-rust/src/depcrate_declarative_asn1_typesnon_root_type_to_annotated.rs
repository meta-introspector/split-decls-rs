// Generated macro for non_root_type_to_annotated (function)
macro_rules! Depcrate_declarative_asn1_typesnon_root_type_to_annotated {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"non_root_type_to_annotated"}
// Dependencies: {}
# [doc = " Utility function for converting builtin Python types."] # [doc = " This is needed when `encode_der` and `decode_der` are called"] # [doc = " with builtin Python types (`int`, `str`, etc), and we can't"] # [doc = " handle the conversion to the Rust `AnnotatedType` like we"] # [doc = " do for classes with `@sequence`."] fn non_root_type_to_annotated < 'p > (py : pyo3 :: Python < 'p > , class : & pyo3 :: Bound < 'p , pyo3 :: types :: PyType > ,) -> pyo3 :: PyResult < AnnotatedType > { let inner = non_root_python_to_rust (py , class) ? . unbind () ; Ok (AnnotatedType { inner , annotation : Annotation { default : None , encoding : None , size : None , } . into_pyobject (py) ? . unbind () , }) }
};
}
