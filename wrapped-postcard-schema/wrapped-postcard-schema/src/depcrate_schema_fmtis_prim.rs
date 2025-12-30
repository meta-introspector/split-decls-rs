// Generated macro for is_prim (function)
macro_rules! Depcrate_schema_fmtis_prim {
() => {
// Module: crate::schema::fmt
// Provides: {"is_prim"}
// Dependencies: {}
# [doc = " Is this [`OwnedDataModelType`] a primitive?"] pub fn is_prim (osdmty : & OwnedDataModelType) -> bool { match osdmty { OwnedDataModelType :: Bool => true , OwnedDataModelType :: I8 => true , OwnedDataModelType :: U8 => true , OwnedDataModelType :: I16 => true , OwnedDataModelType :: I32 => true , OwnedDataModelType :: I64 => true , OwnedDataModelType :: I128 => true , OwnedDataModelType :: U16 => true , OwnedDataModelType :: U32 => true , OwnedDataModelType :: U64 => true , OwnedDataModelType :: U128 => true , OwnedDataModelType :: Usize => true , OwnedDataModelType :: Isize => true , OwnedDataModelType :: F32 => true , OwnedDataModelType :: F64 => true , OwnedDataModelType :: Char => true , OwnedDataModelType :: String => true , OwnedDataModelType :: ByteArray => true , OwnedDataModelType :: Option (owned_named_type) => is_prim (& owned_named_type . ty) , OwnedDataModelType :: Unit => true , OwnedDataModelType :: UnitStruct => true , OwnedDataModelType :: NewtypeStruct (owned_named_type) => is_prim (& owned_named_type . ty) , OwnedDataModelType :: Seq (_) => false , OwnedDataModelType :: Tuple (_) => false , OwnedDataModelType :: TupleStruct (vec) => vec . iter () . all (| e | is_prim (& e . ty)) , OwnedDataModelType :: Map { key , val } => is_prim (& key . ty) && is_prim (& val . ty) , OwnedDataModelType :: Struct (_) => false , OwnedDataModelType :: Enum (_) => false , OwnedDataModelType :: Schema => true , } }
};
}
