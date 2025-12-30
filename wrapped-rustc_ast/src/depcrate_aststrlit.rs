// Generated macro for StrLit (struct)
macro_rules! Depcrate_astStrLit {
() => {
// Module: crate::ast
// Provides: {"StrLit"}
// Dependencies: {}
# [doc = " Similar to `MetaItemLit`, but restricted to string literals."] # [derive (Clone , Copy , Encodable , Decodable , Debug , Walkable)] pub struct StrLit { # [doc = " The original literal as written in source code."] pub symbol : Symbol , # [doc = " The original suffix as written in source code."] pub suffix : Option < Symbol > , # [doc = " The semantic (unescaped) representation of the literal."] pub symbol_unescaped : Symbol , pub style : StrStyle , pub span : Span , }
};
}
