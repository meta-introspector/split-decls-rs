// Generated macro for DelimArgs (struct)
macro_rules! Depcrate_astDelimArgs {
() => {
// Module: crate::ast
// Provides: {"DelimArgs"}
// Dependencies: {}
# [doc = " Delimited arguments, as used in `#[attr()/[]/{}]` or `mac!()/[]/{}`."] # [derive (Clone , Encodable , Decodable , Debug , HashStable_Generic , Walkable)] pub struct DelimArgs { pub dspan : DelimSpan , pub delim : Delimiter , pub tokens : TokenStream , }
};
}
