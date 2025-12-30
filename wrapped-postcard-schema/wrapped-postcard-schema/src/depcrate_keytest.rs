// Generated macro for test (module)
macro_rules! Depcrate_keytest {
() => {
// Module: crate::key
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { key :: Key , schema :: { DataModelType , NamedType } , Schema , } ; # [test] fn matches_old_postcard_rpc_defn () { let old = & NamedType { name : "Key" , ty : & DataModelType :: NewtypeStruct (& NamedType { name : "[T; N]" , ty : & DataModelType :: Tuple (& [& NamedType { name : "u8" , ty : & DataModelType :: U8 , } , & NamedType { name : "u8" , ty : & DataModelType :: U8 , } , & NamedType { name : "u8" , ty : & DataModelType :: U8 , } , & NamedType { name : "u8" , ty : & DataModelType :: U8 , } , & NamedType { name : "u8" , ty : & DataModelType :: U8 , } , & NamedType { name : "u8" , ty : & DataModelType :: U8 , } , & NamedType { name : "u8" , ty : & DataModelType :: U8 , } , & NamedType { name : "u8" , ty : & DataModelType :: U8 , } ,]) , }) , } ; let new = < Key as Schema > :: SCHEMA ; assert_eq ! (old , new) ; } }
};
}
