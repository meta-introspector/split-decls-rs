// Generated macro for impl_28 (impl)
macro_rules! Depcrate_from_idimpl_28 {
() => {
// Module: crate::from_id
// Provides: {"impl_28"}
// Dependencies: {}
impl From < AdtId > for Adt { fn from (id : AdtId) -> Self { match id { AdtId :: StructId (it) => Adt :: Struct (it . into ()) , AdtId :: UnionId (it) => Adt :: Union (it . into ()) , AdtId :: EnumId (it) => Adt :: Enum (it . into ()) , } } }
};
}
