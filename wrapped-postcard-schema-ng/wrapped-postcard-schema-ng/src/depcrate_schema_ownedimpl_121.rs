// Generated macro for impl_121 (impl)
macro_rules! Depcrate_schema_ownedimpl_121 {
() => {
// Module: crate::schema::owned
// Provides: {"impl_121"}
// Dependencies: {}
impl From < & DataModelType > for OwnedDataModelType { fn from (other : & DataModelType) -> Self { match other { DataModelType :: Bool => Self :: Bool , DataModelType :: I8 => Self :: I8 , DataModelType :: U8 => Self :: U8 , DataModelType :: I16 => Self :: I16 , DataModelType :: I32 => Self :: I32 , DataModelType :: I64 => Self :: I64 , DataModelType :: I128 => Self :: I128 , DataModelType :: U16 => Self :: U16 , DataModelType :: U32 => Self :: U32 , DataModelType :: U64 => Self :: U64 , DataModelType :: U128 => Self :: U128 , DataModelType :: Usize => Self :: Usize , DataModelType :: Isize => Self :: Isize , DataModelType :: F32 => Self :: F32 , DataModelType :: F64 => Self :: F64 , DataModelType :: Char => Self :: Char , DataModelType :: String => Self :: String , DataModelType :: ByteArray => Self :: ByteArray , DataModelType :: Option (o) => Self :: Option (Box :: new ((* o) . into ())) , DataModelType :: Unit => Self :: Unit , DataModelType :: Seq (s) => Self :: Seq (Box :: new ((* s) . into ())) , DataModelType :: Tuple (t) => Self :: Tuple (t . iter () . map (| i | (* i) . into ()) . collect ()) , DataModelType :: Map { key , val } => Self :: Map { key : Box :: new ((* key) . into ()) , val : Box :: new ((* val) . into ()) , } , DataModelType :: Struct { name , data } => Self :: Struct { name : (* name) . into () , data : data . into () , } , DataModelType :: Enum { name , variants } => Self :: Enum { name : (* name) . into () , variants : variants . iter () . map (| i | (* i) . into ()) . collect () , } , DataModelType :: Schema => Self :: Schema , } } }
};
}
