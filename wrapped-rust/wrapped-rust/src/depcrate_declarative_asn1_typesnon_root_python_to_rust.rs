// Generated macro for non_root_python_to_rust (function)
macro_rules! Depcrate_declarative_asn1_typesnon_root_python_to_rust {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"non_root_python_to_rust"}
// Dependencies: {}
# [doc = " Utility function for converting builtin Python types"] # [doc = " to their Rust `Type` equivalent."] # [pyo3 :: pyfunction] pub fn non_root_python_to_rust < 'p > (py : pyo3 :: Python < 'p > , class : & pyo3 :: Bound < 'p , pyo3 :: types :: PyType > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , Type > > { if class . is (pyo3 :: types :: PyInt :: type_object (py)) { Type :: PyInt () . into_pyobject (py) } else if class . is (pyo3 :: types :: PyBool :: type_object (py)) { Type :: PyBool () . into_pyobject (py) } else if class . is (pyo3 :: types :: PyString :: type_object (py)) { Type :: PyStr () . into_pyobject (py) } else if class . is (pyo3 :: types :: PyBytes :: type_object (py)) { Type :: PyBytes () . into_pyobject (py) } else if class . is (PrintableString :: type_object (py)) { Type :: PrintableString () . into_pyobject (py) } else if class . is (UtcTime :: type_object (py)) { Type :: UtcTime () . into_pyobject (py) } else if class . is (GeneralizedTime :: type_object (py)) { Type :: GeneralizedTime () . into_pyobject (py) } else { Err (pyo3 :: exceptions :: PyTypeError :: new_err (format ! ("cannot handle type: {class:?}"))) } }
};
}
