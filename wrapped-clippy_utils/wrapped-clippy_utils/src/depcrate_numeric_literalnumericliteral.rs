// Generated macro for NumericLiteral (struct)
macro_rules! Depcrate_numeric_literalNumericLiteral {
() => {
// Module: crate::numeric_literal
// Provides: {"NumericLiteral"}
// Dependencies: {}
# [derive (Debug)] pub struct NumericLiteral < 'a > { # [doc = " Which radix the literal was represented in."] pub radix : Radix , # [doc = " The radix prefix, if present."] pub prefix : Option < & 'a str > , # [doc = " The integer part of the number."] pub integer : & 'a str , # [doc = " The fraction part of the number."] pub fraction : Option < & 'a str > , # [doc = " The exponent separator (b'e' or b'E') including preceding underscore if present"] # [doc = " and the exponent part."] pub exponent : Option < (& 'a str , & 'a str) > , # [doc = " The type suffix, including preceding underscore if present."] pub suffix : Option < & 'a str > , }
};
}
