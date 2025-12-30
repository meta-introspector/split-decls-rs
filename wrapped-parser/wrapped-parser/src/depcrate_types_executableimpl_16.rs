// Generated macro for impl_16 (impl)
macro_rules! Depcrate_types_executableimpl_16 {
() => {
// Module: crate::types::executable
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a > Iterator for OperationsIter < 'a > { type Item = (Option < & 'a Name > , & 'a Positioned < OperationDefinition >) ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . 0 { OperationsIterInner :: Single (op) => op . take () . map (| op | (None , op)) , OperationsIterInner :: Multiple (iter) => iter . next () . map (| (name , op) | (Some (name) , op)) , } } fn size_hint (& self) -> (usize , Option < usize >) { let size = self . len () ; (size , Some (size)) } }
};
}
