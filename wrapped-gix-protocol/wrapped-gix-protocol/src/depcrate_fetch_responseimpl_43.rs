// Generated macro for impl_43 (impl)
macro_rules! Depcrate_fetch_responseimpl_43 {
() => {
// Module: crate::fetch::response
// Provides: {"impl_43"}
// Dependencies: {}
impl From < std :: io :: Error > for Error { fn from (err : std :: io :: Error) -> Self { if err . kind () == std :: io :: ErrorKind :: Other { match err . into_inner () { Some (err) => match err . downcast :: < gix_transport :: packetline :: read :: Error > () { Ok (err) => Error :: UploadPack (* err) , Err (err) => Error :: Io (std :: io :: Error :: other (err)) , } , None => Error :: Io (std :: io :: ErrorKind :: Other . into ()) , } } else { Error :: Io (err) } } }
};
}
