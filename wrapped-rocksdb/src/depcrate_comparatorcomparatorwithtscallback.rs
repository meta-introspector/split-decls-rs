// Generated macro for ComparatorWithTsCallback (struct)
macro_rules! Depcrate_comparatorComparatorWithTsCallback {
() => {
// Module: crate::comparator
// Provides: {"ComparatorWithTsCallback"}
// Dependencies: {}
pub struct ComparatorWithTsCallback { pub name : CString , pub compare_fn : Box < CompareFn > , pub compare_ts_fn : Box < CompareTsFn > , pub compare_without_ts_fn : Box < CompareWithoutTsFn > , }
};
}
