// Generated macro for impl_445 (impl)
macro_rules! Depcrate_declarative_asn1_typesimpl_445 {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"impl_445"}
// Dependencies: {}
# [pyo3 :: pymethods] impl Size { # [new] fn new (min : usize , max : Option < usize >) -> Self { Size { min , max } } # [staticmethod] fn exact (n : usize) -> Self { Size { min : n , max : Some (n) , } } }
};
}
