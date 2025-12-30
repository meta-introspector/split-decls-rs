// Generated macro for macro_8037 (macro)
macro_rules! Depcrate_neg_cmp_op_on_partial_ordmacro_8037 {
() => {
// Module: crate::neg_cmp_op_on_partial_ord
// Provides: {"macro_8037"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the usage of negated comparison operators on types which only implement"] # [doc = " `PartialOrd` (e.g., `f64`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These operators make it easy to forget that the underlying types actually allow not only three"] # [doc = " potential Orderings (Less, Equal, Greater) but also a fourth one (Uncomparable). This is"] # [doc = " especially easy to miss if the operator based comparison result is negated."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = 1.0;"] # [doc = " let b = f64::NAN;"] # [doc = ""] # [doc = " let not_less_or_equal = !(a <= b);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::cmp::Ordering;"] # [doc = " # let a = 1.0;"] # [doc = " # let b = f64::NAN;"] # [doc = ""] # [doc = " let _not_less_or_equal = match a.partial_cmp(&b) {"] # [doc = "     None | Some(Ordering::Greater) => true,"] # [doc = "     _ => false,"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEG_CMP_OP_ON_PARTIAL_ORD , complexity , "The use of negated comparison operators on partially ordered types may produce confusing code." }
};
}
