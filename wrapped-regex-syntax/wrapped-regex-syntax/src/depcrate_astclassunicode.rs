// Generated macro for ClassUnicode (struct)
macro_rules! Depcrate_astClassUnicode {
() => {
// Module: crate::ast
// Provides: {"ClassUnicode"}
// Dependencies: {}
# [doc = " A Unicode character class."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct ClassUnicode { # [doc = " The span of this class."] pub span : Span , # [doc = " Whether this class is negated or not."] # [doc = ""] # [doc = " Note: be careful when using this attribute. This specifically refers"] # [doc = " to whether the class is written as `\\p` or `\\P`, where the latter"] # [doc = " is `negated = true`. However, it also possible to write something like"] # [doc = " `\\P{scx!=Katakana}` which is actually equivalent to"] # [doc = " `\\p{scx=Katakana}` and is therefore not actually negated even though"] # [doc = " `negated = true` here. To test whether this class is truly negated"] # [doc = " or not, use the `is_negated` method."] pub negated : bool , # [doc = " The kind of Unicode class."] pub kind : ClassUnicodeKind , }
};
}
