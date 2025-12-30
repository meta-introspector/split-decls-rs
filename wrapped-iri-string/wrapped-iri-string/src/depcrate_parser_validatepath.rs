// Generated macro for path (function)
macro_rules! Depcrate_parser_validatepath {
() => {
// Module: crate::parser::validate
// Provides: {"path"}
// Dependencies: {}
# [doc = " Validates [IRI path][path]."] # [doc = ""] # [doc = " [path]: https://tools.ietf.org/html/rfc3986#section-3.3"] pub fn path < S : Spec > (s : & str) -> Result < () , Error > { parser :: validate_path :: < S > (s) }
};
}
