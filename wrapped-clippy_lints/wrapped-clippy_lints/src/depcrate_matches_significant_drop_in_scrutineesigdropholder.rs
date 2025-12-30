// Generated macro for SigDropHolder (enum)
macro_rules! Depcrate_matches_significant_drop_in_scrutineeSigDropHolder {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"SigDropHolder"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Debug , Default)] enum SigDropHolder { # [doc = " No values with significant drop present in this expression."] # [doc = ""] # [doc = " Expressions that we've emitted lints do not count."] # [default] None , # [doc = " Some field in this expression references to values with significant drop."] # [doc = ""] # [doc = " Example: `(1, &data.lock().field)`."] PackedRef , # [doc = " The value of this expression references to values with significant drop."] # [doc = ""] # [doc = " Example: `data.lock().field`."] DirectRef , # [doc = " This expression should be moved out to avoid significant drop in scrutinee."] Moved , }
};
}
