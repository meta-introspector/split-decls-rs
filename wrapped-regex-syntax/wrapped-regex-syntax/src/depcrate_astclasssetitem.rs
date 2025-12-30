// Generated macro for ClassSetItem (enum)
macro_rules! Depcrate_astClassSetItem {
() => {
// Module: crate::ast
// Provides: {"ClassSetItem"}
// Dependencies: {}
# [doc = " A single component of a character class set."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum ClassSetItem { # [doc = " An empty item."] # [doc = ""] # [doc = " Note that a bracketed character class cannot contain a single empty"] # [doc = " item. Empty items can appear when using one of the binary operators."] # [doc = " For example, `[&&]` is the intersection of two empty classes."] Empty (Span) , # [doc = " A single literal."] Literal (Literal) , # [doc = " A range between two literals."] Range (ClassSetRange) , # [doc = " An ASCII character class, e.g., `[:alnum:]` or `[:punct:]`."] Ascii (ClassAscii) , # [doc = " A Unicode character class, e.g., `\\pL` or `\\p{Greek}`."] Unicode (ClassUnicode) , # [doc = " A perl character class, e.g., `\\d` or `\\W`."] Perl (ClassPerl) , # [doc = " A bracketed character class set, which may contain zero or more"] # [doc = " character ranges and/or zero or more nested classes. e.g.,"] # [doc = " `[a-zA-Z\\pL]`."] Bracketed (Box < ClassBracketed >) , # [doc = " A union of items."] Union (ClassSetUnion) , }
};
}
