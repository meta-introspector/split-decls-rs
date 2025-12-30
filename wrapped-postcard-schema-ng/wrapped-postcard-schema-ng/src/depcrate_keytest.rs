// Generated macro for test (module)
macro_rules! Depcrate_keytest {
() => {
// Module: crate::key
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { key :: Key , schema :: DataModelType , Schema } ; # [test] fn matches_old_postcard_rpc_defn () { let old = & DataModelType :: Struct { name : "Key" , data : crate :: schema :: Data :: Newtype (& DataModelType :: Tuple (& [& DataModelType :: U8 , & DataModelType :: U8 , & DataModelType :: U8 , & DataModelType :: U8 , & DataModelType :: U8 , & DataModelType :: U8 , & DataModelType :: U8 , & DataModelType :: U8 ,])) , } ; let new = < Key as Schema > :: SCHEMA ; assert_eq ! (old , new) ; } }
};
}
