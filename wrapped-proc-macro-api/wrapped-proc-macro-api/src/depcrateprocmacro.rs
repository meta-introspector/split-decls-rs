// Generated macro for ProcMacro (struct)
macro_rules! DepcrateProcMacro {
() => {
// Module: crate
// Provides: {"ProcMacro"}
// Dependencies: {}
# [doc = " A handle to a specific proc-macro (a `#[proc_macro]` annotated function)."] # [doc = ""] # [doc = " It exists within the context of a specific proc-macro server -- currently"] # [doc = " we share a single expander process for all macros within a workspace."] # [derive (Debug , Clone)] pub struct ProcMacro { process : Arc < ProcMacroServerProcess > , dylib_path : Arc < AbsPathBuf > , name : Box < str > , kind : ProcMacroKind , dylib_last_modified : Option < SystemTime > , }
};
}
