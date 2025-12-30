// Generated macro for simple_case_fold_both_result (function)
macro_rules! Depcratesimple_case_fold_both_result {
() => {
// Module: crate
// Provides: {"simple_case_fold_both_result"}
// Dependencies: {}
# [doc = " The result of binary search on the simple case folding table."] # [doc = ""] # [doc = " Note that this binary search is done on the \"both\" table, such that"] # [doc = " the index returned corresponds to the *first* location of `c1` in the"] # [doc = " table. The table can then be scanned linearly starting from the position"] # [doc = " returned to find other case mappings for `c1`."] fn simple_case_fold_both_result (c1 : char) -> result :: Result < usize , usize > { let table = & case_folding :: C_plus_S_both_table ; let i = binary_search (table , | & (c2 , _) | c1 <= c2) ; if i >= table . len () || table [i] . 0 != c1 { Err (i) } else { Ok (i) } }
};
}
