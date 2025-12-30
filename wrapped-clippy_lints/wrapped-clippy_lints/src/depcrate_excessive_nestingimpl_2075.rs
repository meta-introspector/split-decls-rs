// Generated macro for impl_2075 (impl)
macro_rules! Depcrate_excessive_nestingimpl_2075 {
() => {
// Module: crate::excessive_nesting
// Provides: {"impl_2075"}
// Dependencies: {}
impl NestingVisitor < '_ , '_ > { fn check_indent (& mut self , span : Span , id : NodeId) -> bool { if self . nest_level > self . conf . excessive_nesting_threshold && ! span . in_external_macro (self . cx . sess () . source_map ()) { self . conf . nodes . insert (id) ; return true ; } false } }
};
}
