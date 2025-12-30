// Generated macro for register_lints (function)
macro_rules! Depcrateregister_lints {
() => {
// Module: crate
// Provides: {"register_lints"}
// Dependencies: {}
pub fn register_lints (store : & mut LintStore) { store . register_lints (LINTS) ; store . register_early_pass (| | Box :: new (unsorted_clippy_utils_paths :: UnsortedClippyUtilsPaths)) ; store . register_early_pass (| | Box :: new (produce_ice :: ProduceIce)) ; store . register_late_pass (| _ | Box :: new (collapsible_calls :: CollapsibleCalls)) ; store . register_late_pass (| _ | Box :: new (derive_deserialize_allowing_unknown :: DeriveDeserializeAllowingUnknown)) ; store . register_late_pass (| _ | Box :: < symbols :: Symbols > :: default ()) ; store . register_late_pass (| _ | Box :: < lint_without_lint_pass :: LintWithoutLintPass > :: default ()) ; store . register_late_pass (| _ | Box :: new (unnecessary_def_path :: UnnecessaryDefPath)) ; store . register_late_pass (| _ | Box :: new (outer_expn_data_pass :: OuterExpnDataPass)) ; store . register_late_pass (| _ | Box :: new (msrv_attr_impl :: MsrvAttrImpl)) ; store . register_late_pass (| _ | Box :: new (almost_standard_lint_formulation :: AlmostStandardFormulation :: new ())) ; store . register_late_pass (| _ | Box :: new (unusual_names :: UnusualNames)) ; }
};
}
