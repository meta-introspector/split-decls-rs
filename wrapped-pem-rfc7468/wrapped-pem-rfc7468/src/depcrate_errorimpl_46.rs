// Generated macro for impl_46 (impl)
macro_rules! Depcrate_errorimpl_46 {
() => {
// Module: crate::error
// Provides: {"impl_46"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < Error > for std :: io :: Error { fn from (err : Error) -> std :: io :: Error { let kind = match err { Error :: Base64 (err) => return err . into () , Error :: CharacterEncoding | Error :: EncapsulatedText | Error :: Label | Error :: Preamble | Error :: PreEncapsulationBoundary | Error :: PostEncapsulationBoundary => std :: io :: ErrorKind :: InvalidData , Error :: Length => std :: io :: ErrorKind :: UnexpectedEof , _ => std :: io :: ErrorKind :: Other , } ; std :: io :: Error :: new (kind , err) } }
};
}
