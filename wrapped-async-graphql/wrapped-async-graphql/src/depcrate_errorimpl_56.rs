// Generated macro for impl_56 (impl)
macro_rules! Depcrate_errorimpl_56 {
() => {
// Module: crate::error
// Provides: {"impl_56"}
// Dependencies: {}
impl Error { # [doc = " Create an error from the given error message."] pub fn new (message : impl Into < String >) -> Self { Self { message : message . into () , source : None , extensions : None , } } # [doc = " Create an error with a type that implements `Display`, and it will also"] # [doc = " set the `source` of the error to this value."] pub fn new_with_source (source : impl Display + Send + Sync + 'static) -> Self { Self { message : source . to_string () , source : Some (Arc :: new (source)) , extensions : None , } } # [doc = " Convert the error to a server error."] # [must_use] pub fn into_server_error (self , pos : Pos) -> ServerError { ServerError { message : self . message , source : self . source , locations : vec ! [pos] , path : Vec :: new () , extensions : self . extensions , } } }
};
}
