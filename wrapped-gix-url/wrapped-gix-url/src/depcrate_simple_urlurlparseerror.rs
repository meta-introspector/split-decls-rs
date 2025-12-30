// Generated macro for UrlParseError (enum)
macro_rules! Depcrate_simple_urlUrlParseError {
() => {
// Module: crate::simple_url
// Provides: {"UrlParseError"}
// Dependencies: {}
# [doc = " Minimal parse error type to replace url::ParseError"] # [derive (Debug , Clone , PartialEq , Eq , thiserror :: Error)] # [allow (missing_docs)] pub enum UrlParseError { # [error ("relative URL without a base")] RelativeUrlWithoutBase , # [error ("invalid port number - must be between 1-65535")] InvalidPort , # [error ("invalid domain character")] InvalidDomainCharacter , # [error ("Scheme requires host")] SchemeRequiresHost , }
};
}
