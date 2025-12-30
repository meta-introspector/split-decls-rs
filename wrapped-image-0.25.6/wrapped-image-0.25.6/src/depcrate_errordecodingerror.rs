// Generated macro for DecodingError (struct)
macro_rules! Depcrate_errorDecodingError {
() => {
// Module: crate::error
// Provides: {"DecodingError"}
// Dependencies: {}
# [doc = " An error was encountered while decoding an image."] # [doc = ""] # [doc = " This is used as an opaque representation for the [`ImageError::Decoding`] variant. See its"] # [doc = " documentation for more information."] # [doc = ""] # [doc = " [`ImageError::Decoding`]: enum.ImageError.html#variant.Decoding"] # [derive (Debug)] pub struct DecodingError { format : ImageFormatHint , underlying : Option < Box < dyn Error + Send + Sync > > , }
};
}
