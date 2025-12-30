// Generated macro for UnknownCTargetFeature (struct)
macro_rules! Depcrate_errorsUnknownCTargetFeature {
() => {
// Module: crate::errors
// Provides: {"UnknownCTargetFeature"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_ssa_unknown_ctarget_feature)] # [note] pub (crate) struct UnknownCTargetFeature < 'a > { pub feature : & 'a str , # [subdiagnostic] pub rust_feature : PossibleFeature < 'a > , }
};
}
