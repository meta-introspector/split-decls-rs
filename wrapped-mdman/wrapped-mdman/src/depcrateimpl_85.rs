// Generated macro for impl_85 (impl)
macro_rules! Depcrateimpl_85 {
() => {
// Module: crate
// Provides: {"impl_85"}
// Dependencies: {}
impl Format { # [doc = " The filename extension for the format."] pub fn extension (& self , section : Section) -> String { match self { Format :: Man => section . to_string () , Format :: Md => "md" . to_string () , Format :: Text => "txt" . to_string () , } } }
};
}
