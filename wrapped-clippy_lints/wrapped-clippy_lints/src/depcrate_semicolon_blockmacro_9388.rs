// Generated macro for macro_9388 (macro)
macro_rules! Depcrate_semicolon_blockmacro_9388 {
() => {
// Module: crate::semicolon_block
// Provides: {"macro_9388"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Suggests moving the semicolon after a block to the inside of the block, after its last"] # [doc = " expression."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " For consistency it's best to have the semicolon inside/outside the block. Either way is fine"] # [doc = " and this lint suggests inside the block."] # [doc = " Take a look at `semicolon_outside_block` for the other alternative."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn f(_: u32) {}"] # [doc = " # let x = 0;"] # [doc = " unsafe { f(x) };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn f(_: u32) {}"] # [doc = " # let x = 0;"] # [doc = " unsafe { f(x); }"] # [doc = " ```"] # [clippy :: version = "1.68.0"] pub SEMICOLON_INSIDE_BLOCK , restriction , "add a semicolon inside the block" }
};
}
