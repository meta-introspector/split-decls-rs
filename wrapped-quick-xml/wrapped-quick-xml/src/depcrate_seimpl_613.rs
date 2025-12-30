// Generated macro for impl_613 (impl)
macro_rules! Depcrate_seimpl_613 {
() => {
// Module: crate::se
// Provides: {"impl_613"}
// Dependencies: {}
impl < 'n > XmlName < 'n > { # [doc = " Checks correctness of the XML name according to [XML 1.1 specification]"] # [doc = ""] # [doc = " [XML 1.1 specification]: https://www.w3.org/TR/xml11/#NT-Name"] pub fn try_from (name : & 'n str) -> Result < XmlName < 'n > , SeError > { match name . chars () . next () { Some (ch) if ! is_xml11_name_start_char (ch) => Err (SeError :: Unsupported (format ! ("character `{ch}` is not allowed at the start of an XML name `{name}`") . into () ,)) , _ => match name . matches (| ch | ! is_xml11_name_char (ch)) . next () { Some (s) => Err (SeError :: Unsupported (format ! ("character `{s}` is not allowed in an XML name `{name}`") . into () ,)) , None => Ok (XmlName (name)) , } , } } }
};
}
