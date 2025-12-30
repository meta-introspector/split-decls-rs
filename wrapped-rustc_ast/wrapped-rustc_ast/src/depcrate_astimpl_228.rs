// Generated macro for impl_228 (impl)
macro_rules! Depcrate_astimpl_228 {
() => {
// Module: crate::ast
// Provides: {"impl_228"}
// Dependencies: {}
impl VariantData { # [doc = " Return the fields of this variant."] pub fn fields (& self) -> & [FieldDef] { match self { VariantData :: Struct { fields , .. } | VariantData :: Tuple (fields , _) => fields , _ => & [] , } } # [doc = " Return the `NodeId` of this variant's constructor, if it has one."] pub fn ctor_node_id (& self) -> Option < NodeId > { match * self { VariantData :: Struct { .. } => None , VariantData :: Tuple (_ , id) | VariantData :: Unit (id) => Some (id) , } } }
};
}
