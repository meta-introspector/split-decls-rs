// Generated macro for AngleBracketedArg (enum)
macro_rules! Depcrate_astAngleBracketedArg {
() => {
// Module: crate::ast
// Provides: {"AngleBracketedArg"}
// Dependencies: {}
# [doc = " Either an argument for a generic parameter or a constraint on an associated item."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum AngleBracketedArg { # [doc = " A generic argument for a generic parameter."] Arg (GenericArg) , # [doc = " A constraint on an associated item."] Constraint (AssocItemConstraint) , }
};
}
