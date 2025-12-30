// Generated macro for is_fn_unsafe_to_call (function)
macro_rules! Depcrate_utilsis_fn_unsafe_to_call {
() => {
// Module: crate::utils
// Provides: {"is_fn_unsafe_to_call"}
// Dependencies: {}
pub fn is_fn_unsafe_to_call (db : & dyn HirDatabase , func : FunctionId , caller_target_features : & TargetFeatures , call_edition : Edition , target_feature_is_safe : TargetFeatureIsSafeInTarget ,) -> Unsafety { let data = db . function_signature (func) ; if data . is_unsafe () { return Unsafety :: Unsafe ; } if data . has_target_feature () && target_feature_is_safe == TargetFeatureIsSafeInTarget :: No { let callee_target_features = TargetFeatures :: from_attrs_no_implications (& db . attrs (func . into ())) ; if ! caller_target_features . enabled . is_superset (& callee_target_features . enabled) { return Unsafety :: Unsafe ; } } if data . is_deprecated_safe_2024 () { if call_edition . at_least_2024 () { return Unsafety :: Unsafe ; } else { return Unsafety :: DeprecatedSafe2024 ; } } let loc = func . lookup (db) ; match loc . container { hir_def :: ItemContainerId :: ExternBlockId (block) => { let is_intrinsic_block = block . abi (db) == Some (sym :: rust_dash_intrinsic) ; if is_intrinsic_block { if db . attrs (func . into ()) . by_key (sym :: rustc_safe_intrinsic) . exists () { Unsafety :: Safe } else { Unsafety :: Unsafe } } else { if data . is_safe () { Unsafety :: Safe } else { Unsafety :: Unsafe } } } _ => Unsafety :: Safe , } }
};
}
