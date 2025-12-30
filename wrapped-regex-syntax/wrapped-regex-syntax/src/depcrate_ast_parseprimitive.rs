// Generated macro for Primitive (enum)
macro_rules! Depcrate_ast_parsePrimitive {
() => {
// Module: crate::ast::parse
// Provides: {"Primitive"}
// Dependencies: {}
# [doc = " A primitive is an expression with no sub-expressions. This includes"] # [doc = " literals, assertions and non-set character classes. This representation"] # [doc = " is used as intermediate state in the parser."] # [doc = ""] # [doc = " This does not include ASCII character classes, since they can only appear"] # [doc = " within a set character class."] # [derive (Clone , Debug , Eq , PartialEq)] enum Primitive { Literal (ast :: Literal) , Assertion (ast :: Assertion) , Dot (Span) , Perl (ast :: ClassPerl) , Unicode (ast :: ClassUnicode) , }
};
}
