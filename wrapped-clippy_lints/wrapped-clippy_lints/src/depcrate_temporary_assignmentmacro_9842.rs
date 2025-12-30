// Generated macro for macro_9842 (macro)
macro_rules! Depcrate_temporary_assignmentmacro_9842 {
() => {
// Module: crate::temporary_assignment
// Provides: {"macro_9842"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for construction of a structure or tuple just to"] # [doc = " assign a value in it."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability. If the structure is only created to be"] # [doc = " updated, why not write the structure you want in the first place?"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " (0, 0).0 = 1"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub TEMPORARY_ASSIGNMENT , complexity , "assignments to temporaries" }
};
}
