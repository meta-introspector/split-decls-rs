// Generated macro for impl_130 (impl)
macro_rules! Depcrate_passesimpl_130 {
() => {
// Module: crate::passes
// Provides: {"impl_130"}
// Dependencies: {}
impl LintStoreExpand for LintStoreExpandImpl < '_ > { fn pre_expansion_lint (& self , sess : & Session , features : & Features , registered_tools : & RegisteredTools , node_id : ast :: NodeId , attrs : & [ast :: Attribute] , items : & [Box < ast :: Item >] , name : Symbol ,) { pre_expansion_lint (sess , features , self . 0 , registered_tools , (node_id , attrs , items) , name) ; } }
};
}
