// Generated macro for CondCode (trait)
macro_rules! Depcrate_ir_condcodesCondCode {
() => {
// Module: crate::ir::condcodes
// Provides: {"CondCode"}
// Dependencies: {}
# [doc = " Common traits of condition codes."] pub trait CondCode : Copy { # [doc = " Get the complemented condition code of `self`."] # [doc = ""] # [doc = " The complemented condition code produces the opposite result for all comparisons."] # [doc = " That is, `cmp CC, x, y` is true if and only if `cmp CC.complement(), x, y` is false."] # [must_use] fn complement (self) -> Self ; # [doc = " Get the swapped args condition code for `self`."] # [doc = ""] # [doc = " The swapped args condition code produces the same result as swapping `x` and `y` in the"] # [doc = " comparison. That is, `cmp CC, x, y` is the same as `cmp CC.swap_args(), y, x`."] # [must_use] fn swap_args (self) -> Self ; }
};
}
