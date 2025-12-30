// Generated macro for peel_blocks (function)
macro_rules! Depcratepeel_blocks {
() => {
// Module: crate
// Provides: {"peel_blocks"}
// Dependencies: {}
# [doc = " Removes blocks around an expression, only if the block contains just one expression"] # [doc = " and no statements. Unsafe blocks are not removed."] # [doc = ""] # [doc = " Examples:"] # [doc = "  * `{}`               -> `{}`"] # [doc = "  * `{ x }`            -> `x`"] # [doc = "  * `{{ x }}`          -> `x`"] # [doc = "  * `{ x; }`           -> `{ x; }`"] # [doc = "  * `{ x; y }`         -> `{ x; y }`"] # [doc = "  * `{ unsafe { x } }` -> `unsafe { x }`"] pub fn peel_blocks < 'a > (mut expr : & 'a Expr < 'a >) -> & 'a Expr < 'a > { while let ExprKind :: Block (Block { stmts : [] , expr : Some (inner) , rules : BlockCheckMode :: DefaultBlock , .. } , _ ,) = expr . kind { expr = inner ; } expr }
};
}
