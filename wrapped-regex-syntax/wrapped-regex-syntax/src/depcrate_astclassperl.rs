// Generated macro for ClassPerl (struct)
macro_rules! Depcrate_astClassPerl {
() => {
// Module: crate::ast
// Provides: {"ClassPerl"}
// Dependencies: {}
# [doc = " A Perl character class."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct ClassPerl { # [doc = " The span of this class."] pub span : Span , # [doc = " The kind of Perl class."] pub kind : ClassPerlKind , # [doc = " Whether the class is negated or not. e.g., `\\d` is not negated but"] # [doc = " `\\D` is."] pub negated : bool , }
};
}
