// Generated macro for AdapterType (enum)
macro_rules! Depcrate_wit_standardAdapterType {
() => {
// Module: crate::wit::standard
// Provides: {"AdapterType"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum AdapterType { S8 , S16 , S32 , S64 , S128 , U8 , U16 , U32 , U64 , U128 , F32 , F64 , String , Externref , Bool , I32 , I64 , Vector (VectorKind) , Option (Box < AdapterType >) , Struct (String) , Enum (String) , StringEnum (String) , NamedExternref (String) , Function , NonNull , }
};
}
