// Generated macro for impl_41 (impl)
macro_rules! Depcrate_from_idimpl_41 {
() => {
// Module: crate::from_id
// Provides: {"impl_41"}
// Dependencies: {}
impl From < Adt > for GenericDefId { fn from (id : Adt) -> Self { match id { Adt :: Struct (it) => it . id . into () , Adt :: Union (it) => it . id . into () , Adt :: Enum (it) => it . id . into () , } } }
};
}
