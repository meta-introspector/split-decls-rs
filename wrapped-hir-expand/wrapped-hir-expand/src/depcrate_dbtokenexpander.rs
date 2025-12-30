// Generated macro for TokenExpander (enum)
macro_rules! Depcrate_dbTokenExpander {
() => {
// Module: crate::db
// Provides: {"TokenExpander"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub enum TokenExpander { # [doc = " Old-style `macro_rules` or the new macros 2.0"] DeclarativeMacro (Arc < DeclarativeMacroExpander >) , # [doc = " Stuff like `line!` and `file!`."] BuiltIn (BuiltinFnLikeExpander) , # [doc = " Built-in eagerly expanded fn-like macros (`include!`, `concat!`, etc.)"] BuiltInEager (EagerExpander) , # [doc = " `global_allocator` and such."] BuiltInAttr (BuiltinAttrExpander) , # [doc = " `derive(Copy)` and such."] BuiltInDerive (BuiltinDeriveExpander) , # [doc = " The thing we love the most here in rust-analyzer -- procedural macros."] ProcMacro (CustomProcMacroExpander) , }
};
}
