// Generated macro for impl_1970 (impl)
macro_rules! Depcrate_resultimpl_1970 {
() => {
// Module: crate::result
// Provides: {"impl_1970"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Error :: InvalidCString (ref nul_err) => write ! (f , "{nul_err}") , Error :: DatabaseError (_ , ref e) => write ! (f , "{}" , e . message ()) , Error :: NotFound => f . write_str ("Record not found") , Error :: QueryBuilderError (ref e) => e . fmt (f) , Error :: DeserializationError (ref e) => e . fmt (f) , Error :: SerializationError (ref e) => e . fmt (f) , Error :: RollbackErrorOnCommit { ref rollback_error , ref commit_error , } => { write ! (f , "Transaction rollback failed: {} \
                        (rollback attempted because of failure to commit: {})" , &** rollback_error , &** commit_error) ? ; Ok (()) } Error :: RollbackTransaction => { write ! (f , "You have asked diesel to rollback the transaction") } Error :: BrokenTransactionManager => write ! (f , "The transaction manager is broken") , Error :: AlreadyInTransaction => write ! (f , "Cannot perform this operation while a transaction is open" ,) , Error :: NotInTransaction => { write ! (f , "Cannot perform this operation outside of a transaction" ,) } } } }
};
}
