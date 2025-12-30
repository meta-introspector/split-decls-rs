// Generated macro for impl_32 (impl)
macro_rules! Depcrate_errorimpl_32 {
() => {
// Module: crate::error
// Provides: {"impl_32"}
// Dependencies: {}
impl EncodingError { # [doc = " Create an `EncodingError` that stems from an arbitrary error of an underlying encoder."] pub fn new (format : ImageFormatHint , err : impl Into < Box < dyn Error + Send + Sync > >) -> Self { EncodingError { format , underlying : Some (err . into ()) , } } # [doc = " Create an `EncodingError` for an image format."] # [doc = ""] # [doc = " The error will not contain any further information but is very easy to create."] # [must_use] pub fn from_format_hint (format : ImageFormatHint) -> Self { EncodingError { format , underlying : None , } } # [doc = " Return the image format associated with this error."] # [must_use] pub fn format_hint (& self) -> ImageFormatHint { self . format . clone () } }
};
}
