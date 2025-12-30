// Generated macro for ConstContext (enum)
macro_rules! Depcrate_hirConstContext {
() => {
// Module: crate::hir
// Provides: {"ConstContext"}
// Dependencies: {}
# [doc = " The kind of an item that requires const-checking."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum ConstContext { # [doc = " A `const fn`."] ConstFn , # [doc = " A `static` or `static mut`."] Static (Mutability) , # [doc = " A `const`, associated `const`, or other const context."] # [doc = ""] # [doc = " Other contexts include:"] # [doc = " - Array length expressions"] # [doc = " - Enum discriminants"] # [doc = " - Const generics"] # [doc = ""] # [doc = " For the most part, other contexts are treated just like a regular `const`, so they are"] # [doc = " lumped into the same category."] Const { inline : bool } , }
};
}
