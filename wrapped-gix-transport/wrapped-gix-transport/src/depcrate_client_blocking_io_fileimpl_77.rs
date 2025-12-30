// Generated macro for impl_77 (impl)
macro_rules! Depcrate_client_blocking_io_fileimpl_77 {
() => {
// Module: crate::client::blocking_io::file
// Provides: {"impl_77"}
// Dependencies: {}
impl ReadStdoutFailOnError { fn swap_err_if_present_in_stderr (& self , wanted : usize , res : std :: io :: Result < usize >) -> std :: io :: Result < usize > { match self . recv . try_recv () . ok () { Some (err) => Err (err) , None => match res { Ok (n) if n == wanted => Ok (n) , Ok (n) => { self . recv . recv_timeout (std :: time :: Duration :: from_millis (5)) . ok () . map_or (Ok (n) , Err) } Err (err) => Err (self . recv . recv () . ok () . unwrap_or (err)) , } , } } }
};
}
