// Generated macro for LocalKind (enum)
macro_rules! Depcrate_astLocalKind {
() => {
// Module: crate::ast
// Provides: {"LocalKind"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum LocalKind { # [doc = " Local declaration."] # [doc = " Example: `let x;`"] Decl , # [doc = " Local declaration with an initializer."] # [doc = " Example: `let x = y;`"] Init (Box < Expr >) , # [doc = " Local declaration with an initializer and an `else` clause."] # [doc = " Example: `let Some(x) = y else { return };`"] InitElse (Box < Expr > , Box < Block >) , }
};
}
