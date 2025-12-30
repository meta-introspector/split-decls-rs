// Generated macro for impl_1974 (impl)
macro_rules! Depcrate_resultimpl_1974 {
() => {
// Module: crate::result
// Provides: {"impl_1974"}
// Dependencies: {}
impl PartialEq for Error { fn eq (& self , other : & Error) -> bool { match (self , other) { (Error :: InvalidCString (a) , Error :: InvalidCString (b)) => a == b , (Error :: DatabaseError (_ , a) , Error :: DatabaseError (_ , b)) => a . message () == b . message () , (& Error :: NotFound , & Error :: NotFound) => true , (& Error :: RollbackTransaction , & Error :: RollbackTransaction) => true , (& Error :: AlreadyInTransaction , & Error :: AlreadyInTransaction) => true , _ => false , } } }
};
}
