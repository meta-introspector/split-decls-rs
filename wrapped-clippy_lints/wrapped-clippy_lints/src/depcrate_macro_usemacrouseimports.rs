// Generated macro for MacroUseImports (struct)
macro_rules! Depcrate_macro_useMacroUseImports {
() => {
// Module: crate::macro_use
// Provides: {"MacroUseImports"}
// Dependencies: {}
# [derive (Default)] pub struct MacroUseImports { # [doc = " the actual import path used and the span of the attribute above it. The value is"] # [doc = " the location, where the lint should be emitted."] imports : Vec < (String , Span , hir :: HirId) > , # [doc = " the span of the macro reference, kept to ensure only one reference is used per macro call."] collected : FxHashSet < Span > , mac_refs : Vec < MacroRefData > , }
};
}
