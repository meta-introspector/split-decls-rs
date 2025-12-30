// Generated macro for StructRest (enum)
macro_rules! Depcrate_astStructRest {
() => {
// Module: crate::ast
// Provides: {"StructRest"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum StructRest { # [doc = " `..x`."] Base (Box < Expr >) , # [doc = " `..`."] Rest (Span) , # [doc = " No trailing `..` or expression."] None , }
};
}
