// Generated macro for Context (enum)
macro_rules! Depcrate_percent_encodeContext {
() => {
// Module: crate::percent_encode
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Context for percent encoding."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] enum Context { # [doc = " Encode the string as a reg-name (usually called as \"hostname\")."] RegName , # [doc = " Encode the string as a user name or a password (inside the `userinfo` component)."] UserOrPassword , # [doc = " Encode the string as a path segment."] # [doc = ""] # [doc = " A slash (`/`) will be encoded to `%2F`."] PathSegment , # [doc = " Encode the string as path segments joined with `/`."] # [doc = ""] # [doc = " A slash (`/`) will be used as is."] Path , # [doc = " Encode the string as a query string (without the `?` prefix)."] Query , # [doc = " Encode the string as a fragment string (without the `#` prefix)."] Fragment , # [doc = " Encode all characters except for `unreserved` characters."] Unreserve , # [doc = " Encode characters only if they cannot appear anywhere in an IRI reference."] # [doc = ""] # [doc = " `%` character will be always encoded."] Character , }
};
}
