// Generated macro for impl_815 (impl)
macro_rules! Depcrateimpl_815 {
() => {
// Module: crate
// Provides: {"impl_815"}
// Dependencies: {}
impl < N : AstIdNode > AstIdLoc for ItemLoc < N > { type Container = ModuleId ; type Ast = N ; # [inline] fn ast_id (& self) -> AstId < Self :: Ast > { self . id } # [inline] fn container (& self) -> Self :: Container { self . container } }
};
}
