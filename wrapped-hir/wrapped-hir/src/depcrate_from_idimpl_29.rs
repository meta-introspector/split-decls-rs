// Generated macro for impl_29 (impl)
macro_rules! Depcrate_from_idimpl_29 {
() => {
// Module: crate::from_id
// Provides: {"impl_29"}
// Dependencies: {}
impl From < Adt > for AdtId { fn from (id : Adt) -> Self { match id { Adt :: Struct (it) => AdtId :: StructId (it . id) , Adt :: Union (it) => AdtId :: UnionId (it . id) , Adt :: Enum (it) => AdtId :: EnumId (it . id) , } } }
};
}
