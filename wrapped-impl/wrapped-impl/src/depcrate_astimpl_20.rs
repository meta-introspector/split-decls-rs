// Generated macro for impl_20 (impl)
macro_rules! Depcrate_astimpl_20 {
() => {
// Module: crate::ast
// Provides: {"impl_20"}
// Dependencies: {}
impl ContainerKind { fn from_struct (node : & DataStruct) -> Self { match node . fields { Fields :: Named (_) => ContainerKind :: Struct , Fields :: Unnamed (_) => ContainerKind :: TupleStruct , Fields :: Unit => ContainerKind :: UnitStruct , } } fn from_variant (node : & syn :: Variant) -> Self { match node . fields { Fields :: Named (_) => ContainerKind :: StructVariant , Fields :: Unnamed (_) => ContainerKind :: TupleVariant , Fields :: Unit => ContainerKind :: UnitVariant , } } }
};
}
