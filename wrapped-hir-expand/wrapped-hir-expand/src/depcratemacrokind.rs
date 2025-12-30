// Generated macro for MacroKind (enum)
macro_rules! DepcrateMacroKind {
() => {
// Module: crate
// Provides: {"MacroKind"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum MacroKind { # [doc = " `macro_rules!` or Macros 2.0 macro."] Declarative , # [doc = " A built-in function-like macro."] DeclarativeBuiltIn , # [doc = " A custom derive."] Derive , # [doc = " A builtin-in derive."] DeriveBuiltIn , # [doc = " A procedural attribute macro."] Attr , # [doc = " A built-in attribute macro."] AttrBuiltIn , # [doc = " A function-like procedural macro."] ProcMacro , }
};
}
