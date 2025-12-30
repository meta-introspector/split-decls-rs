// Generated macro for impl_2072 (impl)
macro_rules! Depcrate_excessive_nestingimpl_2072 {
() => {
// Module: crate::excessive_nesting
// Provides: {"impl_2072"}
// Dependencies: {}
impl ExcessiveNesting { pub fn new (conf : & 'static Conf) -> Self { Self { excessive_nesting_threshold : conf . excessive_nesting_threshold , nodes : NodeSet :: default () , } } pub fn check_node_id (& self , cx : & EarlyContext < '_ > , span : Span , node_id : NodeId) { if self . nodes . contains (& node_id) { span_lint_and_help (cx , EXCESSIVE_NESTING , span , "this block is too nested" , None , "try refactoring your code to minimize nesting" ,) ; } } }
};
}
