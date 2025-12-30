// Generated macro for impl_276 (impl)
macro_rules! Depcrate_executorimpl_276 {
() => {
// Module: crate::executor
// Provides: {"impl_276"}
// Dependencies: {}
impl FieldPath < '_ > { fn construct_path (& self , acc : & mut Vec < String >) { match self { FieldPath :: Root (_) => () , FieldPath :: Field (name , _ , parent) => { parent . construct_path (acc) ; acc . push ((* name) . into ()) ; } } } fn location (& self) -> & SourcePosition { match * self { FieldPath :: Root (ref pos) | FieldPath :: Field (_ , ref pos , _) => pos , } } }
};
}
