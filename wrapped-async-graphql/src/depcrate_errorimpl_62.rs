// Generated macro for impl_62 (impl)
macro_rules! Depcrate_errorimpl_62 {
() => {
// Module: crate::error
// Provides: {"impl_62"}
// Dependencies: {}
impl From < multer :: Error > for ParseRequestError { fn from (err : multer :: Error) -> Self { match err { multer :: Error :: FieldSizeExceeded { .. } | multer :: Error :: StreamSizeExceeded { .. } => { ParseRequestError :: PayloadTooLarge } _ => ParseRequestError :: InvalidMultipart (err) , } } }
};
}
