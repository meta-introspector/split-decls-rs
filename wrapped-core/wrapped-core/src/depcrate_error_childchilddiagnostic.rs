// Generated macro for ChildDiagnostic (struct)
macro_rules! Depcrate_error_childChildDiagnostic {
() => {
// Module: crate::error::child
// Provides: {"ChildDiagnostic"}
// Dependencies: {}
# [doc = " Supplemental message for an [`Error`](super::Error) when it's emitted as a `Diagnostic`."] # [doc = ""] # [doc = " # Example Output"] # [doc = " The `note` and `help` lines below come from child diagnostics."] # [doc = ""] # [doc = " ```text"] # [doc = " error: My custom error"] # [doc = "   --> my_project/my_file.rs:3:5"] # [doc = "    |"] # [doc = " 13 |     FooBar { value: String },"] # [doc = "    |     ^^^^^^"] # [doc = "    |"] # [doc = "    = note: My note on the macro usage"] # [doc = "    = help: Try doing this instead"] # [doc = " ```"] # [derive (Debug , Clone)] pub (in crate :: error) struct ChildDiagnostic { level : Level , span : Option < Span > , message : String , }
};
}
