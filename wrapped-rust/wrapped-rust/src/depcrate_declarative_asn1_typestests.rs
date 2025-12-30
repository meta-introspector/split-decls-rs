// Generated macro for tests (module)
macro_rules! Depcrate_declarative_asn1_typestests {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use pyo3 :: IntoPyObject ; use super :: { type_to_tag , AnnotatedType , Annotation , Type } ; # [test] fn test_option_type_to_tag () { pyo3 :: Python :: initialize () ; pyo3 :: Python :: attach (| py | { let ann_type = pyo3 :: Py :: new (py , AnnotatedType { inner : pyo3 :: Py :: new (py , Type :: PyInt ()) . unwrap () , annotation : Annotation { default : None , encoding : None , size : None , } . into_pyobject (py) . unwrap () . unbind () , } ,) . unwrap () ; let optional_type = pyo3 :: Py :: new (py , AnnotatedType { inner : pyo3 :: Py :: new (py , Type :: Option (ann_type)) . unwrap () , annotation : Annotation { default : None , encoding : None , size : None , } . into_pyobject (py) . unwrap () . unbind () , } ,) . unwrap () ; let expected_tag = type_to_tag (& Type :: Option (optional_type) , & None) ; assert_eq ! (expected_tag , type_to_tag (& Type :: PyInt () , & None)) }) } }
};
}
