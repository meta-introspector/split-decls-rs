// Generated macro for get_status (function)
macro_rules! Depcrate_process_unixget_status {
() => {
// Module: crate::process::unix
// Provides: {"get_status"}
// Dependencies: {}
fn get_status (proc : & PtyProcess) -> std :: prelude :: v1 :: Result < WaitStatus , io :: Error > { match proc . status () { Ok (status) => Ok (status) , Err (err) => match err { Errno :: ECHILD | Errno :: ESRCH => Err (io :: Error :: new (ErrorKind :: WouldBlock , err)) , err => Err (io :: Error :: other (err)) , } , } }
};
}
