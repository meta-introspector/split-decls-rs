// Generated macro for impl_810 (impl)
macro_rules! Depcrateimpl_810 {
() => {
// Module: crate
// Provides: {"impl_810"}
// Dependencies: {}
impl < N : AstIdNode > AstIdLoc for ItemLoc < N > { type Container = ModuleId ; type Ast = N ; # [inline] fn ast_id (& self) -> AstId < Self :: Ast > { self . id } # [inline] fn container (& self) -> Self :: Container { self . container } }
};
}
