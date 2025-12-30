// Generated macro for ClassUnicodeOpKind (enum)
macro_rules! Depcrate_astClassUnicodeOpKind {
() => {
// Module: crate::ast
// Provides: {"ClassUnicodeOpKind"}
// Dependencies: {}
# [doc = " The type of op used in a Unicode character class."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum ClassUnicodeOpKind { # [doc = " A property set to a specific value, e.g., `\\p{scx=Katakana}`."] Equal , # [doc = " A property set to a specific value using a colon, e.g.,"] # [doc = " `\\p{scx:Katakana}`."] Colon , # [doc = " A property that isn't a particular value, e.g., `\\p{scx!=Katakana}`."] NotEqual , }
};
}
