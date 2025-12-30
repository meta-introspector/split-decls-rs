// Generated macro for Type (struct)
macro_rules! Depcrate_astType {
() => {
// Module: crate::ast
// Provides: {"Type"}
// Dependencies: {}
# [doc = " Type literal in a syntax tree."] # [doc = ""] # [doc = " Carries no semantic information and might refer to types that don't exist."] # [derive (Clone , Copy , Debug)] pub struct Type < N = ArcStr , M = TypeModifiers > { # [doc = " Name of this [`Type`]."] name : N , # [doc = " Modifiers of this [`Type`]."] # [doc = ""] # [doc = " The first one is the innermost one."] modifiers : M , }
};
}
