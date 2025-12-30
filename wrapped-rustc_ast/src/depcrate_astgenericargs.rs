// Generated macro for GenericArgs (enum)
macro_rules! Depcrate_astGenericArgs {
() => {
// Module: crate::ast
// Provides: {"GenericArgs"}
// Dependencies: {}
# [doc = " The generic arguments and associated item constraints of a path segment."] # [doc = ""] # [doc = " E.g., `<A, B>` as in `Foo<A, B>` or `(A, B)` as in `Foo(A, B)`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum GenericArgs { # [doc = " The `<'a, A, B, C>` in `foo::bar::baz::<'a, A, B, C>`."] AngleBracketed (AngleBracketedArgs) , # [doc = " The `(A, B)` and `C` in `Foo(A, B) -> C`."] Parenthesized (ParenthesizedArgs) , # [doc = " `(..)` in return type notation."] ParenthesizedElided (Span) , }
};
}
