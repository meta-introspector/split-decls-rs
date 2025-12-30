// Generated macro for Binding (struct)
macro_rules! Depcrate_hirBinding {
() => {
// Module: crate::hir
// Provides: {"Binding"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub struct Binding { pub name : Name , pub mode : BindingAnnotation , pub problems : Option < BindingProblems > , # [doc = " Note that this may not be the direct `SyntaxContextId` of the binding's expansion, because transparent"] # [doc = " expansions are attributed to their parent expansion (recursively)."] pub hygiene : HygieneId , }
};
}
