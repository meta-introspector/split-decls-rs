// Generated macro for impl_811 (impl)
macro_rules! Depcrateimpl_811 {
() => {
// Module: crate
// Provides: {"impl_811"}
// Dependencies: {}
impl < N : AstIdNode > AstIdLoc for AssocItemLoc < N > { type Container = ItemContainerId ; type Ast = N ; # [inline] fn ast_id (& self) -> AstId < Self :: Ast > { self . id } # [inline] fn container (& self) -> Self :: Container { self . container } }
};
}
