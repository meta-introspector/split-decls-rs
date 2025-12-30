// Generated macro for macro_9680 (macro)
macro_rules! Depcrate_semicolon_blockmacro_9680 {
() => {
// Module: crate::semicolon_block
// Provides: {"macro_9680"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Suggests moving the semicolon from a block's final expression outside of the block."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " For consistency it's best to have the semicolon inside/outside the block. Either way is fine"] # [doc = " and this lint suggests outside the block."] # [doc = " Take a look at `semicolon_inside_block` for the other alternative."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn f(_: u32) {}"] # [doc = " # let x = 0;"] # [doc = " unsafe { f(x); }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn f(_: u32) {}"] # [doc = " # let x = 0;"] # [doc = " unsafe { f(x) };"] # [doc = " ```"] # [clippy :: version = "1.68.0"] pub SEMICOLON_OUTSIDE_BLOCK , restriction , "add a semicolon outside the block" }
};
}
