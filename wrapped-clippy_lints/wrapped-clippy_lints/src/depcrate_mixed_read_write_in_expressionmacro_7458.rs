// Generated macro for macro_7458 (macro)
macro_rules! Depcrate_mixed_read_write_in_expressionmacro_7458 {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"macro_7458"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for a read and a write to the same variable where"] # [doc = " whether the read occurs before or after the write depends on the evaluation"] # [doc = " order of sub-expressions."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " While [the evaluation order of sub-expressions] is fully specified in Rust,"] # [doc = " it still may be confusing to read an expression where the evaluation order"] # [doc = " affects its behavior."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Code which intentionally depends on the evaluation"] # [doc = " order, or which is correct for any evaluation order."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut x = 0;"] # [doc = ""] # [doc = " let a = {"] # [doc = "     x = 1;"] # [doc = "     1"] # [doc = " } + x;"] # [doc = " // Unclear whether a is 1 or 2."] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let mut x = 0;"] # [doc = " let tmp = {"] # [doc = "     x = 1;"] # [doc = "     1"] # [doc = " };"] # [doc = " let a = tmp + x;"] # [doc = " ```"] # [doc = ""] # [doc = " [order]: (https://doc.rust-lang.org/reference/expressions.html?highlight=subexpression#evaluation-order-of-operands)"] # [clippy :: version = "pre 1.29.0"] pub MIXED_READ_WRITE_IN_EXPRESSION , restriction , "whether a variable read occurs before a write depends on sub-expression evaluation order" }
};
}
