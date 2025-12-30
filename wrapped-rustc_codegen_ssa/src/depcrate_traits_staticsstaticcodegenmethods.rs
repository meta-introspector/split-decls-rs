// Generated macro for StaticCodegenMethods (trait)
macro_rules! Depcrate_traits_staticsStaticCodegenMethods {
() => {
// Module: crate::traits::statics
// Provides: {"StaticCodegenMethods"}
// Dependencies: {}
pub trait StaticCodegenMethods : BackendTypes { fn static_addr_of (& self , cv : Self :: Value , align : Align , kind : Option < & str >) -> Self :: Value ; fn codegen_static (& mut self , def_id : DefId) ; }
};
}
