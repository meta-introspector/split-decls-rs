// Generated macro for MatchKind (enum)
macro_rules! Depcrate_astMatchKind {
() => {
// Module: crate::ast
// Provides: {"MatchKind"}
// Dependencies: {}
# [doc = " The kind of match expression"] # [derive (Clone , Copy , Encodable , Decodable , Debug , PartialEq , Walkable)] pub enum MatchKind { # [doc = " match expr { ... }"] Prefix , # [doc = " expr.match { ... }"] Postfix , }
};
}
