// Generated macro for bitwise_changes (function)
macro_rules! Depcrate_bit_setbitwise_changes {
() => {
// Module: crate::bit_set
// Provides: {"bitwise_changes"}
// Dependencies: {}
# [doc = " Does this bitwise operation change `out_vec`?"] # [inline] fn bitwise_changes < Op > (out_vec : & [Word] , in_vec : & [Word] , op : Op) -> bool where Op : Fn (Word , Word) -> Word , { assert_eq ! (out_vec . len () , in_vec . len ()) ; for (out_elem , in_elem) in iter :: zip (out_vec , in_vec) { let old_val = * out_elem ; let new_val = op (old_val , * in_elem) ; if old_val != new_val { return true ; } } false }
};
}
