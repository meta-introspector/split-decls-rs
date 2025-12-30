// Generated macro for EncodingError (struct)
macro_rules! Depcrate_errorEncodingError {
() => {
// Module: crate::error
// Provides: {"EncodingError"}
// Dependencies: {}
# [doc = " An error was encountered while encoding an image."] # [doc = ""] # [doc = " This is used as an opaque representation for the [`ImageError::Encoding`] variant. See its"] # [doc = " documentation for more information."] # [doc = ""] # [doc = " [`ImageError::Encoding`]: enum.ImageError.html#variant.Encoding"] # [derive (Debug)] pub struct EncodingError { format : ImageFormatHint , underlying : Option < Box < dyn Error + Send + Sync > > , }
};
}
