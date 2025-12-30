// Generated macro for DiagnosticCode (struct)
macro_rules! Depcrate_format_diagnosticDiagnosticCode {
() => {
// Module: crate::format::diagnostic
// Provides: {"DiagnosticCode"}
// Dependencies: {}
# [doc = " The error code associated to this diagnostic."] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [non_exhaustive] pub struct DiagnosticCode < 'a > { # [doc = " The code itself."] # [serde (borrow)] pub code : CowStr < 'a > , # [doc = " An explanation for the code"] # [serde (borrow)] pub explanation : Option < CowStr < 'a > > , }
};
}
