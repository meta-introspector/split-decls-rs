// Generated macro for NSComparisonResult (enum)
macro_rules! Depcrate_comparison_resultNSComparisonResult {
() => {
// Module: crate::comparison_result
// Provides: {"NSComparisonResult"}
// Dependencies: {}
# [doc = " Constants that indicate sort order."] # [doc = ""] # [doc = " See [Apple's documentation](https://developer.apple.com/documentation/foundation/nscomparisonresult?language=objc)."] # [repr (isize)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord)] pub enum NSComparisonResult { # [doc = " The left operand is smaller than the right operand."] Ascending = - 1 , # [doc = " The two operands are equal."] Same = 0 , # [doc = " The left operand is greater than the right operand."] Descending = 1 , }
};
}
