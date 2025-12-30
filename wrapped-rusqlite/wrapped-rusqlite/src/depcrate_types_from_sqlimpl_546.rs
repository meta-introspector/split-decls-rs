// Generated macro for impl_546 (impl)
macro_rules! Depcrate_types_from_sqlimpl_546 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_546"}
// Dependencies: {}
impl PartialEq for FromSqlError { fn eq (& self , other : & Self) -> bool { match (self , other) { (Self :: InvalidType , Self :: InvalidType) => true , (Self :: OutOfRange (n1) , Self :: OutOfRange (n2)) => n1 == n2 , (Self :: InvalidBlobSize { expected_size : es1 , blob_size : bs1 , } , Self :: InvalidBlobSize { expected_size : es2 , blob_size : bs2 , } ,) => es1 == es2 && bs1 == bs2 , (..) => false , } } }
};
}
