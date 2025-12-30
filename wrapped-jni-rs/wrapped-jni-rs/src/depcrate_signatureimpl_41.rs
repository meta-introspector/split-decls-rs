// Generated macro for impl_41 (impl)
macro_rules! Depcrate_signatureimpl_41 {
() => {
// Module: crate::signature
// Provides: {"impl_41"}
// Dependencies: {}
impl FromStr for JavaType { type Err = Error ; fn from_str (s : & str) -> std :: result :: Result < Self , Self :: Err > { parser (parse_type) . parse (s) . map_err (| e | Error :: ParseFailed (format ! ("Failed to parse '{s}': {e}"))) . map (| (res , tail) | { if tail . is_empty () { Ok (res) } else { Err (Error :: ParseFailed (format ! ("Trailing input: '{tail}' while parsing '{s}'"))) } }) ? } }
};
}
