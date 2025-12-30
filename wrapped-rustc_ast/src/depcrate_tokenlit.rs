// Generated macro for Lit (struct)
macro_rules! Depcrate_tokenLit {
() => {
// Module: crate::token
// Provides: {"Lit"}
// Dependencies: {}
# [doc = " A literal token."] # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct Lit { pub kind : LitKind , pub symbol : Symbol , pub suffix : Option < Symbol > , }
};
}
