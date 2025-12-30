// Generated macro for Fields (enum)
macro_rules! Depcrate_schemaFields {
() => {
// Module: crate::schema
// Provides: {"Fields"}
// Dependencies: {}
# [doc = " The collection representing the fields of a struct."] # [derive (Clone , PartialEq , Eq , Debug , BorshSerialize , BorshDeserialize , BorshSchemaMacro)] pub enum Fields { # [doc = " The struct with named fields, structurally identical to a tuple."] # [doc = " `FieldName` is metadata, not present in a type's serialized representation."] NamedFields (Vec < (FieldName , Declaration) >) , # [doc = " The struct with unnamed fields, structurally identical to a tuple."] UnnamedFields (Vec < Declaration >) , # [doc = " The struct with no fields, structurally identical to an empty tuple."] Empty , }
};
}
