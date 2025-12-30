// Generated macro for Parts (struct)
macro_rules! Depcrate_uriParts {
() => {
// Module: crate::uri
// Provides: {"Parts"}
// Dependencies: {}
# [doc = " The various parts of a URI."] # [doc = ""] # [doc = " This struct is used to provide to and retrieve from a URI."] # [derive (Debug , Default)] pub struct Parts { # [doc = " The scheme component of a URI"] pub scheme : Option < Scheme > , # [doc = " The authority component of a URI"] pub authority : Option < Authority > , # [doc = " The origin-form component of a URI"] pub path_and_query : Option < PathAndQuery > , # [doc = " Allow extending in the future"] _priv : () , }
};
}
