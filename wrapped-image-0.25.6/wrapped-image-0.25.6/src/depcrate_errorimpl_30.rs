// Generated macro for impl_30 (impl)
macro_rules! Depcrate_errorimpl_30 {
() => {
// Module: crate::error
// Provides: {"impl_30"}
// Dependencies: {}
impl UnsupportedError { # [doc = " Create an `UnsupportedError` for an image with details on the unsupported feature."] # [doc = ""] # [doc = " If the operation was not connected to a particular image format then the hint may be"] # [doc = " `Unknown`."] # [must_use] pub fn from_format_and_kind (format : ImageFormatHint , kind : UnsupportedErrorKind) -> Self { UnsupportedError { format , kind } } # [doc = " Returns the corresponding `UnsupportedErrorKind` of the error."] # [must_use] pub fn kind (& self) -> UnsupportedErrorKind { self . kind . clone () } # [doc = " Returns the image format associated with this error."] # [must_use] pub fn format_hint (& self) -> ImageFormatHint { self . format . clone () } }
};
}
