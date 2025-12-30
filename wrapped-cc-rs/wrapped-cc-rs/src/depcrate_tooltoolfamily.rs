// Generated macro for ToolFamily (enum)
macro_rules! Depcrate_toolToolFamily {
() => {
// Module: crate::tool
// Provides: {"ToolFamily"}
// Dependencies: {}
# [doc = " Represents the family of tools this tool belongs to."] # [doc = ""] # [doc = " Each family of tools differs in how and what arguments they accept."] # [doc = ""] # [doc = " Detection of a family is done on best-effort basis and may not accurately reflect the tool."] # [derive (Copy , Clone , Debug , PartialEq)] pub enum ToolFamily { # [doc = " Tool is GNU Compiler Collection-like."] Gnu , # [doc = " Tool is Clang-like. It differs from the GCC in a sense that it accepts superset of flags"] # [doc = " and its cross-compilation approach is different."] Clang { zig_cc : bool } , # [doc = " Tool is the MSVC cl.exe."] Msvc { clang_cl : bool } , }
};
}
