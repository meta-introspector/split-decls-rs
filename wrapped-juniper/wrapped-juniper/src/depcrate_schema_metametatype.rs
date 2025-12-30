// Generated macro for MetaType (enum)
macro_rules! Depcrate_schema_metaMetaType {
() => {
// Module: crate::schema::meta
// Provides: {"MetaType"}
// Dependencies: {}
# [doc = " Generic type metadata"] # [derive (Debug)] pub enum MetaType < S = DefaultScalarValue > { # [doc (hidden)] Scalar (ScalarMeta < S >) , # [doc (hidden)] List (ListMeta) , # [doc (hidden)] Nullable (NullableMeta) , # [doc (hidden)] Object (ObjectMeta < S >) , # [doc (hidden)] Enum (EnumMeta < S >) , # [doc (hidden)] Interface (InterfaceMeta < S >) , # [doc (hidden)] Union (UnionMeta) , # [doc (hidden)] InputObject (InputObjectMeta < S >) , # [doc (hidden)] Placeholder (PlaceholderMeta) , }
};
}
