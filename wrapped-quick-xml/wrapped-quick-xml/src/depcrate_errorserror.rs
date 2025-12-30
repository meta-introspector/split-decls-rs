// Generated macro for Error (enum)
macro_rules! Depcrate_errorsError {
() => {
// Module: crate::errors
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error type used by this crate."] # [derive (Clone , Debug)] pub enum Error { # [doc = " XML document cannot be read from underlying source."] # [doc = ""] # [doc = " Contains the reference-counted I/O error to make the error type `Clone`able."] Io (Arc < IoError >) , # [doc = " The document does not corresponds to the XML grammar."] Syntax (SyntaxError) , # [doc = " The document is not [well-formed](https://www.w3.org/TR/xml11/#dt-wellformed)."] IllFormed (IllFormedError) , # [doc = " Attribute parsing error"] InvalidAttr (AttrError) , # [doc = " Encoding error"] Encoding (EncodingError) , # [doc = " Escape error"] Escape (EscapeError) , # [doc = " Parsed XML has some namespace-related problems"] Namespace (NamespaceError) , }
};
}
