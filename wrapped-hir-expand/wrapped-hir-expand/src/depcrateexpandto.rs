// Generated macro for ExpandTo (enum)
macro_rules! DepcrateExpandTo {
() => {
// Module: crate
// Provides: {"ExpandTo"}
// Dependencies: {}
# [doc = " In Rust, macros expand token trees to token trees. When we want to turn a"] # [doc = " token tree into an AST node, we need to figure out what kind of AST node we"] # [doc = " want: something like `foo` can be a type, an expression, or a pattern."] # [doc = ""] # [doc = " Naively, one would think that \"what this expands to\" is a property of a"] # [doc = " particular macro: macro `m1` returns an item, while macro `m2` returns an"] # [doc = " expression, etc. That's not the case -- macros are polymorphic in the"] # [doc = " result, and can expand to any type of the AST node."] # [doc = ""] # [doc = " What defines the actual AST node is the syntactic context of the macro"] # [doc = " invocation. As a contrived example, in `let T![*] = T![*];` the first `T`"] # [doc = " expands to a pattern, while the second one expands to an expression."] # [doc = ""] # [doc = " `ExpandTo` captures this bit of information about a particular macro call"] # [doc = " site."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ExpandTo { Statements , Items , Pattern , Type , Expr , }
};
}
