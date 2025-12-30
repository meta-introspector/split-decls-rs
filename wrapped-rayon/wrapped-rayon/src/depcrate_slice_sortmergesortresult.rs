// Generated macro for MergeSortResult (enum)
macro_rules! Depcrate_slice_sortMergeSortResult {
() => {
// Module: crate::slice::sort
// Provides: {"MergeSortResult"}
// Dependencies: {}
# [doc = " The result of merge sort."] # [must_use] # [derive (Clone , Copy , PartialEq , Eq)] enum MergeSortResult { # [doc = " The slice has already been sorted."] NonDescending , # [doc = " The slice has been descending and therefore it was left intact."] Descending , # [doc = " The slice was sorted."] Sorted , }
};
}
