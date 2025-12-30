// Generated macro for impl_19 (impl)
macro_rules! Depcrate_errorimpl_19 {
() => {
// Module: crate::error
// Provides: {"impl_19"}
// Dependencies: {}
impl From < Error > for std :: io :: Error { fn from (err : Error) -> Self { use std :: io :: Error as IoError ; use std :: io :: ErrorKind ; match err { Error :: Win (err) => IoError :: from (err) , Error :: Timeout (time) => { IoError :: new (ErrorKind :: TimedOut , format ! ("timeout reached ({:?})" , time)) } Error :: InputClosed => IoError :: new (ErrorKind :: NotFound , String :: from ("Input to console was already closed") ,) , Error :: WaitFailed (wait_event) => IoError :: new (ErrorKind :: Interrupted , format ! ("Waiting for process failed. WAIT_EVENT: {:?}" , wait_event) ,) , } } }
};
}
