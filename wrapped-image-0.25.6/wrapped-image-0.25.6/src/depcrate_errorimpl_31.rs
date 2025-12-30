// Generated macro for impl_31 (impl)
macro_rules! Depcrate_errorimpl_31 {
() => {
// Module: crate::error
// Provides: {"impl_31"}
// Dependencies: {}
impl DecodingError { # [doc = " Create a `DecodingError` that stems from an arbitrary error of an underlying decoder."] pub fn new (format : ImageFormatHint , err : impl Into < Box < dyn Error + Send + Sync > >) -> Self { DecodingError { format , underlying : Some (err . into ()) , } } # [doc = " Create a `DecodingError` for an image format."] # [doc = ""] # [doc = " The error will not contain any further information but is very easy to create."] # [must_use] pub fn from_format_hint (format : ImageFormatHint) -> Self { DecodingError { format , underlying : None , } } # [doc = " Returns the image format associated with this error."] # [must_use] pub fn format_hint (& self) -> ImageFormatHint { self . format . clone () } }
};
}
