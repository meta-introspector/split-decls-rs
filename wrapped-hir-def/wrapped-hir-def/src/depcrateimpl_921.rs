// Generated macro for impl_921 (impl)
macro_rules! Depcrateimpl_921 {
() => {
// Module: crate
// Provides: {"impl_921"}
// Dependencies: {}
impl From < VariantId > for AttrDefId { fn from (vid : VariantId) -> Self { match vid { VariantId :: EnumVariantId (id) => id . into () , VariantId :: StructId (id) => id . into () , VariantId :: UnionId (id) => id . into () , } } }
};
}
