// Generated macro for InternKind (enum)
macro_rules! Depcrate_interpret_internInternKind {
() => {
// Module: crate::interpret::intern
// Provides: {"InternKind"}
// Dependencies: {}
# [doc = " How a constant value should be interned."] # [derive (Copy , Clone , Debug , PartialEq , Hash , Eq)] pub enum InternKind { # [doc = " The `mutability` of the static, ignoring the type which may have interior mutability."] Static (hir :: Mutability) , # [doc = " A `const` item"] Constant , Promoted , }
};
}
