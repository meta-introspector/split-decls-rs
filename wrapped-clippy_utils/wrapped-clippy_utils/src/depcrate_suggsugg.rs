// Generated macro for Sugg (enum)
macro_rules! Depcrate_suggSugg {
() => {
// Module: crate::sugg
// Provides: {"Sugg"}
// Dependencies: {}
# [doc = " A helper type to build suggestion correctly handling parentheses."] # [derive (Clone , Debug , PartialEq)] pub enum Sugg < 'a > { # [doc = " An expression that never needs parentheses such as `1337` or `[0; 42]`."] NonParen (Cow < 'a , str >) , # [doc = " An expression that does not fit in other variants."] MaybeParen (Cow < 'a , str >) , # [doc = " A binary operator expression, including `as`-casts and explicit type"] # [doc = " coercion."] BinOp (AssocOp , Cow < 'a , str > , Cow < 'a , str >) , # [doc = " A unary operator expression. This is used to sometimes represent `!`"] # [doc = " or `-`, but only if the type with and without the operator is kept identical."] # [doc = " It means that doubling the operator can be used to remove it instead, in"] # [doc = " order to provide better suggestions."] UnOp (UnOp , Box < Self >) , }
};
}
