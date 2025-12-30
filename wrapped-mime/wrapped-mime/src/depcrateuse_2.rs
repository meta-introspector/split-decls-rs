// Generated macro for use_2 (pub_use)
macro_rules! Depcrateuse_2 {
() => {
// Module: crate
// Provides: {"use_2"}
// Dependencies: {}
# [doc = " Compile-time `MediaType`s."] # [doc = ""] # [doc = " Performs validation and construction of a `MediaType` at compile-time,"] # [doc = " catching parse errors early, and allowing them to be used as constants"] # [doc = " or statics."] # [doc = ""] # [doc = " This requires the `macro` feature enabled on the mime crate. Something"] # [doc = " like this in your `Cargo.toml`:"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " mime = { version = \"0.4\", features = [\"macro\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " const VND_MYAPP: mime::MediaType = mime::media_type!(\"application/vnd.myapp+json\");"] # [doc = " ```"] # [cfg (feature = "macro")] # [proc_macro_hack] pub use mime_macro :: media_type ;
};
}
