// Generated macro for MergeOperatorCallback (struct)
macro_rules! Depcrate_merge_operatorMergeOperatorCallback {
() => {
// Module: crate::merge_operator
// Provides: {"MergeOperatorCallback"}
// Dependencies: {}
pub struct MergeOperatorCallback < F : MergeFn , PF : MergeFn > { pub name : CString , pub full_merge_fn : F , pub partial_merge_fn : PF , }
};
}
