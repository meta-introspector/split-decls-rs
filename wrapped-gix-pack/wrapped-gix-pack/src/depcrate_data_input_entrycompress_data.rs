// Generated macro for compress_data (function)
macro_rules! Depcrate_data_input_entrycompress_data {
() => {
// Module: crate::data::input::entry
// Provides: {"compress_data"}
// Dependencies: {}
fn compress_data (obj : & gix_object :: Data < '_ >) -> Result < Vec < u8 > , input :: Error > { let mut out = gix_features :: zlib :: stream :: deflate :: Write :: new (Vec :: new ()) ; if let Err (err) = std :: io :: copy (& mut & * obj . data , & mut out) { match err . kind () { std :: io :: ErrorKind :: Other => return Err (input :: Error :: Io (err . into ())) , err => { unreachable ! ("Should never see other errors than zlib, but got {:?}" , err) } } } out . flush () . expect ("zlib flush should never fail") ; Ok (out . into_inner ()) }
};
}
