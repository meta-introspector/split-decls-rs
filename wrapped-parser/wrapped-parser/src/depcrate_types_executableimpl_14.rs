// Generated macro for impl_14 (impl)
macro_rules! Depcrate_types_executableimpl_14 {
() => {
// Module: crate::types::executable
// Provides: {"impl_14"}
// Dependencies: {}
impl DocumentOperations { # [doc = " Iterate over the operations of the document."] # [must_use] pub fn iter (& self) -> OperationsIter < '_ > { OperationsIter (match self { Self :: Single (op) => OperationsIterInner :: Single (Some (op)) , Self :: Multiple (ops) => OperationsIterInner :: Multiple (ops . iter ()) , }) } }
};
}
