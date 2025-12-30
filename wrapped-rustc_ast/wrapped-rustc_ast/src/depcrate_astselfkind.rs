// Generated macro for SelfKind (enum)
macro_rules! Depcrate_astSelfKind {
() => {
// Module: crate::ast
// Provides: {"SelfKind"}
// Dependencies: {}
# [doc = " Alternative representation for `Arg`s describing `self` parameter of methods."] # [doc = ""] # [doc = " E.g., `&mut self` as in `fn foo(&mut self)`."] # [derive (Clone , Encodable , Decodable , Debug)] pub enum SelfKind { # [doc = " `self`, `mut self`"] Value (Mutability) , # [doc = " `&'lt self`, `&'lt mut self`"] Region (Option < Lifetime > , Mutability) , # [doc = " `&'lt pin const self`, `&'lt pin mut self`"] Pinned (Option < Lifetime > , Mutability) , # [doc = " `self: TYPE`, `mut self: TYPE`"] Explicit (Box < Ty > , Mutability) , }
};
}
