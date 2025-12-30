// Generated macro for impl_547 (impl)
macro_rules! Depcrate_types_from_sqlimpl_547 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_547"}
// Dependencies: {}
impl fmt :: Display for FromSqlError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: InvalidType => write ! (f , "Invalid type") , Self :: OutOfRange (i) => write ! (f , "Value {i} out of range") , Self :: InvalidBlobSize { expected_size , blob_size , } => { write ! (f , "Cannot read {expected_size} byte value out of {blob_size} byte blob") } Self :: Other (ref err) => err . fmt (f) , } } }
};
}
