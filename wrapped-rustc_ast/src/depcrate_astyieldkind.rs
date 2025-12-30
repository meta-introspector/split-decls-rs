// Generated macro for YieldKind (enum)
macro_rules! Depcrate_astYieldKind {
() => {
// Module: crate::ast
// Provides: {"YieldKind"}
// Dependencies: {}
# [doc = " The kind of yield expression"] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum YieldKind { # [doc = " yield expr { ... }"] Prefix (Option < Box < Expr > >) , # [doc = " expr.yield { ... }"] Postfix (Box < Expr >) , }
};
}
