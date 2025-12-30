// Generated macro for macro_8528 (macro)
macro_rules! Depcrate_operatorsmacro_8528 {
() => {
// Module: crate::operators
// Provides: {"macro_8528"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for bit masks in comparisons which can be removed"] # [doc = " without changing the outcome. The basic structure can be seen in the"] # [doc = " following table:"] # [doc = ""] # [doc = " |Comparison| Bit Op   |Example     |equals |"] # [doc = " |----------|----------|------------|-------|"] # [doc = " |`>` / `<=`|`\\|` / `^`|`x \\| 2 > 3`|`x > 3`|"] # [doc = " |`<` / `>=`|`\\|` / `^`|`x ^ 1 < 4` |`x < 4`|"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Not equally evil as [`bad_bit_mask`](#bad_bit_mask),"] # [doc = " but still a bit misleading, because the bit mask is ineffective."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " False negatives: This lint will only match instances"] # [doc = " where we have figured out the math (which is for a power-of-two compared"] # [doc = " value). This means things like `x | 1 >= 7` (which would be better written"] # [doc = " as `x >= 6`) will not be reported (but bit masks like this are fairly"] # [doc = " uncommon)."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " if (x | 1 > 3) {  }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " if (x >= 2) {  }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub INEFFECTIVE_BIT_MASK , correctness , "expressions where a bit mask will be rendered useless by a comparison, e.g., `(x | 1) > 2`" }
};
}
