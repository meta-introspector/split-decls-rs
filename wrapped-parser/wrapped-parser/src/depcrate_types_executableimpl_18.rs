// Generated macro for impl_18 (impl)
macro_rules! Depcrate_types_executableimpl_18 {
() => {
// Module: crate::types::executable
// Provides: {"impl_18"}
// Dependencies: {}
impl ExactSizeIterator for OperationsIter < '_ > { fn len (& self) -> usize { match & self . 0 { OperationsIterInner :: Single (opt) => usize :: from (opt . is_some ()) , OperationsIterInner :: Multiple (iter) => iter . len () , } } }
};
}
