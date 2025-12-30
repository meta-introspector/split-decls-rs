// Generated macro for io (module)
macro_rules! Depcrateio {
() => {
// Module: crate
// Provides: {"io"}
// Dependencies: {}
# [doc = " deprecated io module the original io module has been renamed to `image_reader`"] pub mod io { # [deprecated (note = "this type has been moved and renamed to image::ImageReader")] # [doc = " Deprecated re-export of `ImageReader` as `Reader`"] pub type Reader < R > = super :: ImageReader < R > ; # [deprecated (note = "this type has been moved to image::Limits")] # [doc = " Deprecated re-export of `Limits`"] pub type Limits = super :: Limits ; # [deprecated (note = "this type has been moved to image::LimitSupport")] # [doc = " Deprecated re-export of `LimitSupport`"] pub type LimitSupport = super :: LimitSupport ; }
};
}
