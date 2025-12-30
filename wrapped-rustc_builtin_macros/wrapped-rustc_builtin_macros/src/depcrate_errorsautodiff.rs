// Generated macro for autodiff (module)
macro_rules! Depcrate_errorsautodiff {
() => {
// Module: crate::errors
// Provides: {"autodiff"}
// Dependencies: {}
mod autodiff { use super :: * ; # [derive (Diagnostic)] # [diag (builtin_macros_autodiff_missing_config)] pub (crate) struct AutoDiffMissingConfig { # [primary_span] pub (crate) span : Span , } # [derive (Diagnostic)] # [diag (builtin_macros_autodiff_unknown_activity)] pub (crate) struct AutoDiffUnknownActivity { # [primary_span] pub (crate) span : Span , pub (crate) act : String , } # [derive (Diagnostic)] # [diag (builtin_macros_autodiff_ty_activity)] pub (crate) struct AutoDiffInvalidTypeForActivity { # [primary_span] pub (crate) span : Span , pub (crate) act : String , } # [derive (Diagnostic)] # [diag (builtin_macros_autodiff_number_activities)] pub (crate) struct AutoDiffInvalidNumberActivities { # [primary_span] pub (crate) span : Span , pub (crate) expected : usize , pub (crate) found : usize , } # [derive (Diagnostic)] # [diag (builtin_macros_autodiff_mode_activity)] pub (crate) struct AutoDiffInvalidApplicationModeAct { # [primary_span] pub (crate) span : Span , pub (crate) mode : String , pub (crate) act : String , } # [derive (Diagnostic)] # [diag (builtin_macros_autodiff_ret_activity)] pub (crate) struct AutoDiffInvalidRetAct { # [primary_span] pub (crate) span : Span , pub (crate) mode : String , pub (crate) act : String , } # [derive (Diagnostic)] # [diag (builtin_macros_autodiff_width)] pub (crate) struct AutoDiffInvalidWidth { # [primary_span] pub (crate) span : Span , pub (crate) width : u128 , } # [derive (Diagnostic)] # [diag (builtin_macros_autodiff)] pub (crate) struct AutoDiffInvalidApplication { # [primary_span] pub (crate) span : Span , } }
};
}
