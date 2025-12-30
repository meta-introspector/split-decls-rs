// Generated macro for OverflowingUInt (struct)
macro_rules! Depcrate_lintsOverflowingUInt {
() => {
// Module: crate::lints
// Provides: {"OverflowingUInt"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_overflowing_uint)] # [note] pub (crate) struct OverflowingUInt < 'a > { pub ty : & 'a str , pub lit : String , pub min : u128 , pub max : u128 , }
};
}
