// Generated macro for impl_484 (impl)
macro_rules! Depcrate_parser_combinatorimpl_484 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_484"}
// Dependencies: {}
impl < P , R > Factory < P , R > { fn parser < Input > (& mut self , input : & mut Input) -> & mut R where P : FnMut (& mut Input) -> R , { if let Some (ref mut r) = self . 1 { return r ; } self . 1 = Some ((self . 0) (input)) ; self . 1 . as_mut () . unwrap () } }
};
}
