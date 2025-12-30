// Generated macro for impl_2101 (impl)
macro_rules! Depcrate_excessive_nestingimpl_2101 {
() => {
// Module: crate::excessive_nesting
// Provides: {"impl_2101"}
// Dependencies: {}
impl EarlyLintPass for ExcessiveNesting { fn check_crate (& mut self , cx : & EarlyContext < '_ > , krate : & Crate) { if self . excessive_nesting_threshold == 0 { return ; } let mut visitor = NestingVisitor { conf : self , cx , nest_level : 0 , } ; for item in & krate . items { visitor . visit_item (item) ; } } fn check_block (& mut self , cx : & EarlyContext < '_ > , block : & Block) { self . check_node_id (cx , block . span , block . id) ; } fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { self . check_node_id (cx , item . span , item . id) ; } }
};
}
